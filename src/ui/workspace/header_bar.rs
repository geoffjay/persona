use gpui::*;
use gpui_component::{ActiveTheme, InteractiveElementExt};

pub struct HeaderBar;

impl HeaderBar {
    pub fn new() -> Self {
        Self
    }
}

impl Render for HeaderBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("header-bar")
            .w_full()
            .h(px(32.))
            .flex_shrink_0()
            .bg(cx.theme().sidebar)
            .border_b_1()
            .border_color(cx.theme().border)
            .on_double_click(|_event, window: &mut Window, _cx| {
                window.zoom_window();
            })
    }
}
