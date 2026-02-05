use gpui::*;
use gpui_component::ActiveTheme;

pub struct FooterBar;

impl FooterBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for FooterBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("footer-bar")
            .w_full()
            .h(px(32.))
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_t_1()
            .border_color(cx.theme().border)
    }
}
