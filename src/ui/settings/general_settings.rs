use crate::config::AppConfig;
use crate::ui::theme::{apply_theme, get_theme_names};
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::form::{field, v_form};
use gpui_component::select::{Select, SelectState};
use gpui_component::{h_flex, label::Label, v_flex, IndexPath};

pub struct GeneralSettingsPanel {
    config_theme: String,
    theme_select: Option<Entity<SelectState<Vec<String>>>>,
}

impl GeneralSettingsPanel {
    pub fn new() -> Self {
        let app_config = AppConfig::load();
        Self {
            config_theme: app_config.general.theme,
            theme_select: None,
        }
    }

    fn initialize(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let theme_names = get_theme_names();

        // Find the index of the current theme
        let current_theme_index = theme_names
            .iter()
            .position(|t| t == &self.config_theme)
            .map(IndexPath::new);

        self.theme_select = Some(cx.new(|cx| {
            SelectState::new(theme_names, current_theme_index, window, cx)
        }));
    }

    fn is_dirty(&self, cx: &Context<Self>) -> bool {
        if let Some(ref select) = self.theme_select {
            if let Some(theme) = select.read(cx).selected_value() {
                if *theme != self.config_theme {
                    return true;
                }
            }
        }
        false
    }

    fn save_config(&mut self, cx: &mut Context<Self>) {
        if let Some(ref select) = self.theme_select {
            select.update(cx, |state, _cx| {
                if let Some(theme) = state.selected_value() {
                    self.config_theme = theme.clone();
                }
            });
        }

        // Load the full app config, update general section, and save
        let mut app_config = AppConfig::load();
        app_config.general.theme = self.config_theme.clone();
        if let Err(e) = app_config.save() {
            eprintln!("Failed to save config: {}", e);
        }

        // Apply the theme immediately
        apply_theme(&self.config_theme, cx);

        cx.notify();
    }
}

impl Render for GeneralSettingsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Initialize on first render
        if self.theme_select.is_none() {
            self.initialize(window, cx);
        }

        let entity = cx.entity().clone();
        let is_dirty = self.is_dirty(cx);

        let mut header = h_flex()
            .justify_between()
            .items_center()
            .child(Label::new("General").text_xl());

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

        if let Some(ref select) = self.theme_select {
            form = form.child(
                field()
                    .label("Theme")
                    .description("Color scheme for the application")
                    .child(Select::new(select).w(px(200.))),
            );
        }

        v_flex()
            .id("general-settings")
            .size_full()
            .p_4()
            .gap_4()
            .child(header)
            .child(div().flex_1().overflow_hidden().child(form))
    }
}
