use gpui::*;
use gpui_component::{label::Label, v_flex};

pub struct GeneralSettingsPanel;

impl GeneralSettingsPanel {
    pub fn new() -> Self {
        Self
    }
}

impl Render for GeneralSettingsPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("general-settings")
            .size_full()
            .p_4()
            .gap_4()
            .child(Label::new("General").text_xl())
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Application settings will be configured here.").text_sm()),
            )
    }
}
