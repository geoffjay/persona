use crate::config::{AppConfig, PersonasConfig};
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::form::{field, v_form};
use gpui_component::input::{Input, InputState};
use gpui_component::{h_flex, label::Label, v_flex};

pub struct PersonasSettingsPanel {
    config: PersonasConfig,
    directory_input: Option<Entity<InputState>>,
}

impl PersonasSettingsPanel {
    pub fn new() -> Self {
        let app_config = AppConfig::load();
        Self {
            config: app_config.personas,
            directory_input: None,
        }
    }

    fn initialize(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.directory_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Personas directory...")
                .default_value(self.config.directory.to_string_lossy().to_string())
        }));
    }

    fn is_dirty(&self, cx: &Context<Self>) -> bool {
        if let Some(ref input) = self.directory_input {
            let current = input.read(cx).text().to_string();
            if current != self.config.directory.to_string_lossy() {
                return true;
            }
        }
        false
    }

    fn save_config(&mut self, cx: &mut Context<Self>) {
        if let Some(ref input) = self.directory_input {
            input.update(cx, |state, _cx| {
                self.config.directory = state.text().to_string().into();
            });
        }

        let mut app_config = AppConfig::load();
        app_config.personas = self.config.clone();
        if let Err(e) = app_config.save() {
            eprintln!("Failed to save config: {}", e);
        }

        cx.notify();
    }
}

impl Render for PersonasSettingsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.directory_input.is_none() {
            self.initialize(window, cx);
        }

        let entity = cx.entity().clone();
        let is_dirty = self.is_dirty(cx);

        let mut header = h_flex()
            .justify_between()
            .items_center()
            .child(Label::new("Personas").text_xl());

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

        let mut form = v_form();

        if let Some(ref input) = self.directory_input {
            form = form.child(
                field()
                    .label("Directory")
                    .description("Path to the directory containing persona files")
                    .child(Input::new(input).w_full()),
            );
        }

        v_flex()
            .id("personas-settings")
            .size_full()
            .p_4()
            .gap_4()
            .child(header)
            .child(div().flex_1().overflow_hidden().child(form))
    }
}
