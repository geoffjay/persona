use crate::config::{AppConfig, BerryConfig};
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::form::{field, v_form};
use gpui_component::input::{Input, InputState};
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme};
use std::path::PathBuf;

pub struct MemorySettingsPanel {
    berry_config: BerryConfig,
    knowledgebase_dir: PathBuf,
    berry_url_input: Option<Entity<InputState>>,
    kb_directory_input: Option<Entity<InputState>>,
}

impl MemorySettingsPanel {
    pub fn new() -> Self {
        let app_config = AppConfig::load();
        let knowledgebase_dir = app_config
            .personas
            .directory
            .parent()
            .map(|p| p.join("knowledgebase"))
            .unwrap_or_else(|| PathBuf::from("knowledgebase"));

        Self {
            berry_config: app_config.berry,
            knowledgebase_dir,
            berry_url_input: None,
            kb_directory_input: None,
        }
    }

    fn initialize(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.berry_url_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Berry server URL...")
                .default_value(&self.berry_config.server_url)
        }));

        self.kb_directory_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Knowledgebase directory...")
                .default_value(self.knowledgebase_dir.to_string_lossy().to_string())
        }));
    }

    fn is_dirty(&self, cx: &Context<Self>) -> bool {
        if let Some(ref input) = self.berry_url_input {
            let current = input.read(cx).text().to_string();
            if current != self.berry_config.server_url {
                return true;
            }
        }

        if let Some(ref input) = self.kb_directory_input {
            let current = input.read(cx).text().to_string();
            if current != self.knowledgebase_dir.to_string_lossy() {
                return true;
            }
        }

        false
    }

    fn save_config(&mut self, cx: &mut Context<Self>) {
        if let Some(ref input) = self.berry_url_input {
            input.update(cx, |state, _cx| {
                self.berry_config.server_url = state.text().to_string();
            });
        }

        if let Some(ref input) = self.kb_directory_input {
            input.update(cx, |state, _cx| {
                self.knowledgebase_dir = state.text().to_string().into();
            });
        }

        let mut app_config = AppConfig::load();
        app_config.berry = self.berry_config.clone();
        if let Err(e) = app_config.save() {
            eprintln!("Failed to save config: {}", e);
        }

        cx.notify();
    }
}

impl Render for MemorySettingsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.berry_url_input.is_none() {
            self.initialize(window, cx);
        }

        let entity = cx.entity().clone();
        let is_dirty = self.is_dirty(cx);

        let mut header = h_flex()
            .justify_between()
            .items_center()
            .child(Label::new("Memory").text_xl());

        if is_dirty {
            header = header.child(
                Button::new("save")
                    .primary()
                    .label("Save Changes")
                    .on_click(move |_, _window, cx| {
                        entity.update(cx, |this, cx| {
                            this.save_config(cx);
                        });
                    }),
            );
        }

        // Berry panel
        let mut berry_form = v_form();
        if let Some(ref input) = self.berry_url_input {
            berry_form = berry_form.child(
                field()
                    .label("Server URL")
                    .description("URL of the Berry memory server")
                    .child(Input::new(input).w_full()),
            );
        }

        let berry_panel = v_flex()
            .gap_3()
            .p_4()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border)
            .child(Label::new("Berry").text_lg())
            .child(berry_form);

        // Knowledgebase panel
        let mut kb_form = v_form();
        if let Some(ref input) = self.kb_directory_input {
            kb_form = kb_form.child(
                field()
                    .label("Directory")
                    .description("Path to the directory containing knowledgebase files")
                    .child(Input::new(input).w_full()),
            );
        }

        let kb_panel = v_flex()
            .gap_3()
            .p_4()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().border)
            .child(Label::new("Knowledgebase").text_lg())
            .child(kb_form);

        v_flex()
            .id("memory-settings")
            .size_full()
            .p_4()
            .gap_4()
            .child(header)
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(v_flex().gap_4().child(berry_panel).child(kb_panel)),
            )
    }
}
