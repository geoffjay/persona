use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{h_flex, label::Label, ActiveTheme, IconName, Sizable};

#[derive(Debug, Clone)]
pub enum TerminalHeaderBarEvent {
    ToggleExpanded,
}

impl EventEmitter<TerminalHeaderBarEvent> for TerminalHeaderBar {}

pub struct TerminalHeaderBar {
    persona_name: SharedString,
    is_expanded: bool,
}

impl TerminalHeaderBar {
    pub fn new(persona_name: impl Into<SharedString>, is_expanded: bool) -> Self {
        Self {
            persona_name: persona_name.into(),
            is_expanded,
        }
    }

    pub fn set_expanded(&mut self, is_expanded: bool) {
        self.is_expanded = is_expanded;
    }
}

impl Render for TerminalHeaderBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let icon = if self.is_expanded {
            IconName::PanelLeft
        } else {
            IconName::Maximize
        };

        let tooltip = if self.is_expanded {
            "Collapse sidebar"
        } else {
            "Expand terminal"
        };

        let toggle_button = Button::new("toggle-expand")
            .icon(icon)
            .ghost()
            .small()
            .tooltip(tooltip)
            .on_click(cx.listener(|_this, _, _window, cx| {
                cx.emit(TerminalHeaderBarEvent::ToggleExpanded);
            }));

        h_flex()
            .id("terminal-header-bar")
            .w_full()
            .h(px(36.))
            .px_3()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().title_bar)
            .child(Label::new(self.persona_name.clone()).text_sm())
            .child(toggle_button)
    }
}
