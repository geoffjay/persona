use crate::terminal::AppTerminalConfig;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::form::{field, v_form};
use gpui_component::input::{Input, InputState};
use gpui_component::select::{Select, SelectState};
use gpui_component::{h_flex, label::Label, v_flex, IndexPath};

pub struct TerminalSettingsPanel {
    config: AppTerminalConfig,
    font_family_input: Option<Entity<InputState>>,
    font_size_input: Option<Entity<InputState>>,
    scrollback_input: Option<Entity<InputState>>,
    line_height_input: Option<Entity<InputState>>,
    padding_input: Option<Entity<InputState>>,
    theme_select: Option<Entity<SelectState<Vec<&'static str>>>>,
}

impl TerminalSettingsPanel {
    pub fn new() -> Self {
        Self {
            config: AppTerminalConfig::load(),
            font_family_input: None,
            font_size_input: None,
            scrollback_input: None,
            line_height_input: None,
            padding_input: None,
            theme_select: None,
        }
    }

    fn initialize(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create input states
        self.font_family_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Font family...")
                .default_value(&self.config.font_family)
        }));

        self.font_size_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Font size...")
                .default_value(&self.config.font_size.to_string())
        }));

        self.scrollback_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Scrollback lines...")
                .default_value(&self.config.scrollback.to_string())
        }));

        self.line_height_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Line height...")
                .default_value(&self.config.line_height.to_string())
        }));

        self.padding_input = Some(cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Padding...")
                .default_value(&self.config.padding.to_string())
        }));

        // Create theme select with simple strings
        let themes: Vec<&'static str> = vec!["tokyo-night", "gruvbox", "catppuccin"];

        let current_theme_index = match &self.config.theme {
            crate::terminal::TerminalThemeConfig::Named(name) => {
                themes.iter().position(|t| *t == name.as_str()).map(IndexPath::new)
            }
            _ => None,
        };

        self.theme_select = Some(cx.new(|cx| {
            SelectState::new(themes, current_theme_index, window, cx)
        }));
    }

    /// Check if any input values differ from the saved config
    fn is_dirty(&self, cx: &Context<Self>) -> bool {
        if let Some(ref input) = self.font_family_input {
            let current = input.read(cx).text().to_string();
            if current != self.config.font_family {
                return true;
            }
        }

        if let Some(ref input) = self.font_size_input {
            let current = input.read(cx).text().to_string();
            if current != self.config.font_size.to_string() {
                return true;
            }
        }

        if let Some(ref input) = self.scrollback_input {
            let current = input.read(cx).text().to_string();
            if current != self.config.scrollback.to_string() {
                return true;
            }
        }

        if let Some(ref input) = self.line_height_input {
            let current = input.read(cx).text().to_string();
            if current != self.config.line_height.to_string() {
                return true;
            }
        }

        if let Some(ref input) = self.padding_input {
            let current = input.read(cx).text().to_string();
            if current != self.config.padding.to_string() {
                return true;
            }
        }

        if let Some(ref select) = self.theme_select {
            if let Some(theme) = select.read(cx).selected_value() {
                let current_theme = match &self.config.theme {
                    crate::terminal::TerminalThemeConfig::Named(name) => name.as_str(),
                    _ => "",
                };
                if *theme != current_theme {
                    return true;
                }
            }
        }

        false
    }

    fn save_config(&mut self, cx: &mut Context<Self>) {
        // Read values from inputs
        if let Some(ref input) = self.font_family_input {
            input.update(cx, |state, _cx| {
                self.config.font_family = state.text().to_string();
            });
        }

        if let Some(ref input) = self.font_size_input {
            input.update(cx, |state, _cx| {
                if let Ok(size) = state.text().to_string().parse::<f32>() {
                    self.config.font_size = size;
                }
            });
        }

        if let Some(ref input) = self.scrollback_input {
            input.update(cx, |state, _cx| {
                if let Ok(lines) = state.text().to_string().parse::<usize>() {
                    self.config.scrollback = lines;
                }
            });
        }

        if let Some(ref input) = self.line_height_input {
            input.update(cx, |state, _cx| {
                if let Ok(height) = state.text().to_string().parse::<f32>() {
                    self.config.line_height = height;
                }
            });
        }

        if let Some(ref input) = self.padding_input {
            input.update(cx, |state, _cx| {
                if let Ok(pad) = state.text().to_string().parse::<f32>() {
                    self.config.padding = pad;
                }
            });
        }

        if let Some(ref select) = self.theme_select {
            select.update(cx, |state, _cx| {
                if let Some(theme) = state.selected_value() {
                    self.config.theme =
                        crate::terminal::TerminalThemeConfig::Named(theme.to_string());
                }
            });
        }

        // Save to file
        if let Some(config_path) = AppTerminalConfig::config_path() {
            if let Some(parent) = config_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(toml_str) = toml::to_string_pretty(&self.config) {
                let _ = std::fs::write(&config_path, toml_str);
            }
        }

        cx.notify();
    }
}

impl Render for TerminalSettingsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Initialize on first render
        if self.font_family_input.is_none() {
            self.initialize(window, cx);
        }

        let entity = cx.entity().clone();
        let is_dirty = self.is_dirty(cx);

        let mut header = h_flex()
            .justify_between()
            .items_center()
            .child(Label::new("Terminal").text_xl());

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

        if let Some(ref input) = self.font_family_input {
            form = form.child(
                field()
                    .label("Font Family")
                    .description("The font to use in the terminal")
                    .child(Input::new(input).w_full()),
            );
        }

        if let Some(ref input) = self.font_size_input {
            form = form.child(
                field()
                    .label("Font Size")
                    .description("Font size in points")
                    .child(Input::new(input).w(px(120.))),
            );
        }

        if let Some(ref select) = self.theme_select {
            form = form.child(
                field()
                    .label("Theme")
                    .description("Color scheme for the terminal")
                    .child(Select::new(select).w(px(200.))),
            );
        }

        if let Some(ref input) = self.scrollback_input {
            form = form.child(
                field()
                    .label("Scrollback Lines")
                    .description("Maximum number of lines to keep in history")
                    .child(Input::new(input).w(px(120.))),
            );
        }

        if let Some(ref input) = self.line_height_input {
            form = form.child(
                field()
                    .label("Line Height")
                    .description("Multiplier for line height (e.g., 1.2)")
                    .child(Input::new(input).w(px(120.))),
            );
        }

        if let Some(ref input) = self.padding_input {
            form = form.child(
                field()
                    .label("Padding")
                    .description("Padding around terminal content in pixels")
                    .child(Input::new(input).w(px(120.))),
            );
        }

        v_flex()
            .id("terminal-settings")
            .size_full()
            .p_4()
            .gap_4()
            .child(header)
            .child(div().flex_1().overflow_hidden().child(form))
    }
}
