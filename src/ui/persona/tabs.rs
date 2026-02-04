use crate::state::ConversationTab;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{ActiveTheme, IconName, Sizable};

pub struct ConversationTabs {
    pub tabs: Vec<ConversationTab>,
    pub active_index: Option<usize>,
    pub on_select: Box<dyn Fn(usize, &mut Window, &mut Context<Self>) + 'static>,
    pub on_close: Box<dyn Fn(usize, &mut Window, &mut Context<Self>) + 'static>,
}

impl ConversationTabs {
    pub fn new<S, C>(
        tabs: Vec<ConversationTab>,
        active_index: Option<usize>,
        on_select: S,
        on_close: C,
    ) -> Self
    where
        S: Fn(usize, &mut Window, &mut Context<Self>) + 'static,
        C: Fn(usize, &mut Window, &mut Context<Self>) + 'static,
    {
        Self {
            tabs,
            active_index,
            on_select: Box::new(on_select),
            on_close: Box::new(on_close),
        }
    }
}

impl Render for ConversationTabs {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_index = self.active_index.unwrap_or(0);

        let mut tab_bar = TabBar::new("conversation-tabs")
            .selected_index(active_index)
            .on_click(cx.listener(move |this, index: &usize, window, cx| {
                (this.on_select)(*index, window, cx);
            }));

        for (index, tab) in self.tabs.iter().enumerate() {
            let close_button = Button::new(("close", index))
                .icon(IconName::Close)
                .ghost()
                .xsmall()
                .on_click(cx.listener(move |this, _, window, cx| {
                    (this.on_close)(index, window, cx);
                }));

            let tab_element = Tab::new()
                .label(tab.persona_name.clone())
                .suffix(close_button);

            tab_bar = tab_bar.child(tab_element);
        }

        div()
            .id("conversation-tabs")
            .w_full()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(tab_bar)
    }
}
