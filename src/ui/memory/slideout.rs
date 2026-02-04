use crate::memory::Memory;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex, v_flex, ActiveTheme, IconName, Sizable,
};
use std::sync::Arc;

pub struct MemorySlideout {
    memory: Option<Memory>,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + Send + Sync + 'static>,
}

impl MemorySlideout {
    pub fn new<F>(on_close: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + Send + Sync + 'static,
    {
        Self {
            memory: None,
            on_close: Arc::new(on_close),
        }
    }

    pub fn set_memory(&mut self, memory: Option<Memory>) {
        self.memory = memory;
    }

    fn render_header(&self, memory: &Memory, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        let id_preview: String = memory.id.chars().take(12).collect();

        h_flex()
            .w_full()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(cx.theme().border)
            .justify_between()
            .child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(format!("Memory: {}...", id_preview)),
            )
            .child(
                Button::new("close-slideout")
                    .icon(IconName::Close)
                    .ghost()
                    .xsmall()
                    .on_click(move |_, window, cx| {
                        let on_close = entity.read(cx).on_close.clone();
                        on_close(window, cx);
                    }),
            )
    }

    fn render_section(
        &self,
        title: impl Into<SharedString>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        div()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(cx.theme().muted_foreground)
            .mb_2()
            .child(title.into())
    }

    fn render_content(&self, memory: &Memory, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("slideout-content")
            .w_full()
            .flex_1()
            .p_4()
            .gap_4()
            .overflow_y_scroll()
            // Document section
            .child(
                v_flex()
                    .w_full()
                    .child(self.render_section("Document", cx))
                    .child(
                        div()
                            .w_full()
                            .p_3()
                            .rounded_md()
                            .bg(cx.theme().sidebar)
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(memory.content.clone()),
                    ),
            )
            // Metadata section
            .child(
                v_flex()
                    .w_full()
                    .child(self.render_section("Metadata", cx))
                    .child(self.render_metadata_row("ID", &memory.id, cx))
                    .child(
                        self.render_metadata_row(
                            "Created",
                            &memory
                                .created_at
                                .format("%Y-%m-%d %H:%M:%S UTC")
                                .to_string(),
                            cx,
                        ),
                    )
                    .child(self.render_metadata_row("Author", &memory.created_by, cx))
                    .child(self.render_metadata_row("Type", &memory.memory_type.to_string(), cx))
                    .child(self.render_metadata_row(
                        "Tags",
                        &if memory.tags.is_empty() {
                            "None".to_string()
                        } else {
                            memory.tags.join(", ")
                        },
                        cx,
                    )),
            )
            // Related Memories placeholder
            .child(
                v_flex()
                    .w_full()
                    .child(self.render_section("Related Memories", cx))
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child("Coming soon..."),
                    ),
            )
    }

    fn render_metadata_row(
        &self,
        label: &str,
        value: &str,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .py_1()
            .child(
                div()
                    .w(px(80.))
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(label.to_string()),
            )
            .child(div().flex_1().text_sm().child(value.to_string()))
    }
}

impl Render for MemorySlideout {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(memory) = &self.memory else {
            return div().into_any_element();
        };

        v_flex()
            .w(px(400.))
            .h_full()
            .border_l_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(self.render_header(memory, cx))
            .child(self.render_content(memory, cx))
            .into_any_element()
    }
}
