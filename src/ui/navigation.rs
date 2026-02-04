use crate::state::NavigationView;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{ActiveTheme, IconName, Selectable};

pub struct NavigationBar {
    pub current_view: NavigationView,
    pub on_view_change: Box<dyn Fn(NavigationView, &mut Window, &mut Context<Self>) + 'static>,
}

impl NavigationBar {
    pub fn new<F>(current_view: NavigationView, on_view_change: F) -> Self
    where
        F: Fn(NavigationView, &mut Window, &mut Context<Self>) + 'static,
    {
        Self {
            current_view,
            on_view_change: Box::new(on_view_change),
        }
    }

    fn nav_button(
        &self,
        id: impl Into<ElementId>,
        icon: IconName,
        view: NavigationView,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.current_view == view;
        let entity = cx.entity().clone();

        let mut btn = Button::new(id).ghost().icon(icon);
        if is_active {
            btn = btn.selected(true);
        }
        btn.on_click(move |_, w, cx| {
            entity.update(cx, |this, cx| {
                (this.on_view_change)(view, w, cx);
            });
        })
    }
}

impl Render for NavigationBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("navigation-bar")
            .h_full()
            .w(px(48.))
            .flex_shrink_0()
            .border_r_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().sidebar)
            .flex()
            .flex_col()
            .items_center()
            .py_2()
            .gap_1()
            .child(self.nav_button(
                "nav-personas",
                IconName::User,
                NavigationView::Personas,
                window,
                cx,
            ))
            .child(self.nav_button(
                "nav-memory",
                IconName::Bot,
                NavigationView::Memory,
                window,
                cx,
            ))
            .child(self.nav_button(
                "nav-settings",
                IconName::Settings,
                NavigationView::Settings,
                window,
                cx,
            ))
    }
}
