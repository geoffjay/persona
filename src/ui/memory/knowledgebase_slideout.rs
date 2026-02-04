use crate::knowledgebase::{self, KnowledgebaseFile};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    v_flex, ActiveTheme, IconName, Sizable,
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct KnowledgebaseSlideout {
    file: Option<KnowledgebaseFile>,
    file_path: Option<PathBuf>,
    editor_state: Option<Entity<InputState>>,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + Send + Sync + 'static>,
    save_status: Option<SaveStatus>,
}

#[derive(Debug, Clone)]
enum SaveStatus {
    Saving,
    Saved,
    Error(String),
}

impl KnowledgebaseSlideout {
    pub fn new<F>(on_close: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + Send + Sync + 'static,
    {
        Self {
            file: None,
            file_path: None,
            editor_state: None,
            on_close: Arc::new(on_close),
            save_status: None,
        }
    }

    pub fn set_file(
        &mut self,
        file: Option<KnowledgebaseFile>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.file_path = file.as_ref().map(|f| f.entry.file_path.clone());
        self.save_status = None;

        if let Some(file) = &file {
            let editor = cx.new(|cx| {
                InputState::new(window, cx)
                    .code_editor("markdown")
                    .line_number(true)
                    .default_value(&file.content)
            });
            self.editor_state = Some(editor);
        } else {
            self.editor_state = None;
        }

        self.file = file;
    }

    fn save_file(&mut self, cx: &mut Context<Self>) {
        let Some(path) = &self.file_path else {
            return;
        };
        let Some(editor) = &self.editor_state else {
            return;
        };

        let content = editor.read(cx).text().to_string();
        let path = path.clone();

        self.save_status = Some(SaveStatus::Saving);
        cx.notify();

        match knowledgebase::save_file(&path, &content) {
            Ok(()) => {
                self.save_status = Some(SaveStatus::Saved);
                cx.notify();
            }
            Err(e) => {
                self.save_status = Some(SaveStatus::Error(e.to_string()));
                cx.notify();
            }
        }
    }

    fn render_header(&self, file: &KnowledgebaseFile, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        let entity_for_save = cx.entity().clone();

        h_flex()
            .w_full()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(cx.theme().border)
            .justify_between()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(file.entry.name.clone()),
                    )
                    .when_some(self.save_status.clone(), |this, status| {
                        let (text, color) = match status {
                            SaveStatus::Saving => ("Saving...", cx.theme().muted_foreground),
                            SaveStatus::Saved => ("Saved", cx.theme().success),
                            SaveStatus::Error(_) => ("Error", cx.theme().danger),
                        };
                        this.child(div().text_sm().text_color(color).child(text.to_string()))
                    }),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("save-kb").label("Save").small().on_click(
                        move |_, _window, cx| {
                            entity_for_save.update(cx, |this, cx| {
                                this.save_file(cx);
                            });
                        },
                    ))
                    .child(
                        Button::new("close-kb-slideout")
                            .icon(IconName::Close)
                            .ghost()
                            .xsmall()
                            .on_click(move |_, window, cx| {
                                let on_close = entity.read(cx).on_close.clone();
                                on_close(window, cx);
                            }),
                    ),
            )
    }

    fn render_content(
        &self,
        _file: &KnowledgebaseFile,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(editor) = &self.editor_state else {
            return div().into_any_element();
        };

        v_flex()
            .id("kb-editor-content")
            .w_full()
            .flex_1()
            .p_4()
            .overflow_hidden()
            .child(Input::new(editor).h_full().w_full())
            .into_any_element()
    }
}

impl Render for KnowledgebaseSlideout {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(file) = &self.file else {
            return div().into_any_element();
        };

        v_flex()
            .w(px(600.))
            .h_full()
            .border_l_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(self.render_header(file, cx))
            .child(self.render_content(file, cx))
            .into_any_element()
    }
}
