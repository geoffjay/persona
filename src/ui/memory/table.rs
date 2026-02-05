use crate::memory::Memory;
use gpui::*;
use gpui_component::{h_flex, list::ListItem, v_flex, ActiveTheme};

pub struct MemoryTable {
    memories: Vec<Memory>,
    selected_index: Option<usize>,
    on_select: Box<dyn Fn(usize, &Memory, &mut Window, &mut App) + 'static>,
}

impl MemoryTable {
    pub fn new<F>(memories: Vec<Memory>, on_select: F) -> Self
    where
        F: Fn(usize, &Memory, &mut Window, &mut App) + 'static,
    {
        Self {
            memories,
            selected_index: None,
            on_select: Box::new(on_select),
        }
    }

    pub fn set_memories(&mut self, memories: Vec<Memory>) {
        self.memories = memories;
        self.selected_index = None;
    }

    pub fn set_selected(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().muted)
            .child(
                div()
                    .w(px(200.))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("ID"),
            )
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Document"),
            )
            .child(
                div()
                    .w(px(140.))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Created"),
            )
            .child(
                div()
                    .w(px(200.))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Author"),
            )
            .child(
                div()
                    .w(px(200.))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Tags"),
            )
            .child(
                div()
                    .w(px(80.))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Type"),
            )
    }

    fn render_row(
        &self,
        index: usize,
        memory: &Memory,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected_index == Some(index);
        let memory_clone = memory.clone();
        let entity = cx.entity().clone();

        // Format date
        let created = memory.created_at.format("%Y-%m-%d %H:%M").to_string();

        // Truncate content for display
        let content_preview: String = memory.content.chars().take(60).collect::<String>()
            + if memory.content.len() > 60 { "..." } else { "" };

        // Format tags
        let tags_display = if memory.tags.is_empty() {
            "-".to_string()
        } else {
            memory.tags.join(", ")
        };

        // Truncate ID for display
        let id_display: String = memory.id.chars().take(8).collect();

        ListItem::new(("memory-row", index))
            .px_3()
            .py_1()
            .selected(is_selected)
            .on_click(move |_, window, cx| {
                entity.update(cx, |this, cx| {
                    this.selected_index = Some(index);
                    (this.on_select)(index, &memory_clone, window, cx);
                    cx.notify();
                });
            })
            .child(
                h_flex()
                    .w_full()
                    .gap_0()
                    .child(
                        div()
                            .w(px(200.))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .overflow_hidden()
                            .child(id_display),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .overflow_hidden()
                            .child(content_preview),
                    )
                    .child(
                        div()
                            .w(px(140.))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(created),
                    )
                    .child(
                        div()
                            .w(px(200.))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .overflow_hidden()
                            .child(memory.created_by.clone()),
                    )
                    .child(
                        div()
                            .w(px(200.))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .overflow_hidden()
                            .child(tags_display),
                    )
                    .child(
                        div()
                            .w(px(80.))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(memory.memory_type.to_string()),
                    ),
            )
    }
}

impl Render for MemoryTable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows: Vec<_> = self
            .memories
            .iter()
            .enumerate()
            .map(|(i, m)| self.render_row(i, m, cx))
            .collect();

        v_flex()
            .id("memory-table")
            .w_full()
            .flex_1()
            .overflow_hidden()
            .child(self.render_header(cx))
            .child(
                v_flex()
                    .id("memory-table-rows")
                    .w_full()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}
