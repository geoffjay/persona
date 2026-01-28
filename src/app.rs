use crate::config::AppConfig;
use crate::persona::Persona;
use crate::state::{AppState, NavigationView};
use crate::ui::{ConversationTabs, ConversationView, NavigationBar, PersonaList, SettingsView};
use gpui::*;
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme};
use std::collections::HashMap;

pub struct App {
    state: AppState,
    nav_bar: Entity<NavigationBar>,
    persona_list: Entity<PersonaList>,
    tabs: Entity<ConversationTabs>,
    conversations: HashMap<String, Entity<ConversationView>>,
    settings_view: Entity<SettingsView>,
}

impl App {
    pub fn new(config: AppConfig, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let personas = config.load_personas();
        let state = AppState::new(personas.clone());

        let view = cx.entity().clone();

        let nav_bar = cx.new(|_cx| {
            let view = view.clone();
            NavigationBar::new(state.current_view, move |nav_view, _window, cx| {
                view.update(cx, |this, cx| {
                    this.state.current_view = nav_view;
                    cx.notify();
                });
            })
        });

        let persona_list = cx.new(|_cx| {
            let view = view.clone();
            PersonaList::new(personas, move |persona, window, cx| {
                view.update(cx, |this, inner_cx| {
                    this.open_conversation(persona, window, inner_cx);
                });
            })
        });

        let tabs = cx.new(|_cx| {
            let view_select = view.clone();
            let view_close = view.clone();
            ConversationTabs::new(
                state.open_tabs.clone(),
                state.active_tab_index,
                move |index, _window, cx| {
                    view_select.update(cx, |this, inner_cx| {
                        this.state.set_active_tab(index);
                        this.sync_tabs(inner_cx);
                        inner_cx.notify();
                    });
                },
                move |index, _window, cx| {
                    view_close.update(cx, |this, inner_cx| {
                        this.close_conversation(index, inner_cx);
                    });
                },
            )
        });

        let settings_view = cx.new(|cx| SettingsView::new(_window, cx));

        Self {
            state,
            nav_bar,
            persona_list,
            tabs,
            conversations: HashMap::new(),
            settings_view,
        }
    }

    fn open_conversation(&mut self, persona: &Persona, window: &mut Window, cx: &mut Context<Self>) {
        self.state.open_conversation(persona);

        // Create conversation view if it doesn't exist
        if !self.conversations.contains_key(&persona.id) {
            let persona_clone = persona.clone();
            let conv = cx.new(|cx| ConversationView::new(persona_clone, window, cx));
            self.conversations.insert(persona.id.clone(), conv);
        }

        self.sync_tabs(cx);
        cx.notify();
    }

    fn close_conversation(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(tab) = self.state.open_tabs.get(index) {
            let persona_id = tab.persona_id.clone();
            self.conversations.remove(&persona_id);
        }

        self.state.close_tab(index);
        self.sync_tabs(cx);
        cx.notify();
    }

    fn sync_tabs(&self, cx: &mut Context<Self>) {
        let tabs = self.tabs.clone();
        let open_tabs = self.state.open_tabs.clone();
        let active_index = self.state.active_tab_index;
        cx.defer(move |cx| {
            tabs.update(cx, |tabs_inner, inner_cx| {
                tabs_inner.tabs = open_tabs;
                tabs_inner.active_index = active_index;
                inner_cx.notify();
            });
        });
    }

    fn render_main_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.state.current_view {
            NavigationView::Personas => self.render_personas_view(cx),
            NavigationView::Memory => self.render_placeholder_view("Memory View", cx),
            NavigationView::Settings => self.settings_view.clone().into_any_element(),
        }
    }

    fn render_personas_view(&self, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .size_full()
            .child(self.persona_list.clone())
            .child(self.render_conversation_area(cx))
            .into_any_element()
    }

    fn render_conversation_area(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_tabs = !self.state.open_tabs.is_empty();

        let mut container = v_flex().flex_1().h_full();
        if has_tabs {
            container = container.child(self.tabs.clone());
        }
        container.child(self.render_active_conversation(cx))
    }

    fn render_active_conversation(&self, cx: &mut Context<Self>) -> AnyElement {
        if let Some(index) = self.state.active_tab_index {
            if let Some(tab) = self.state.open_tabs.get(index) {
                if let Some(conv) = self.conversations.get(&tab.persona_id) {
                    return conv.clone().into_any_element();
                }
            }
        }

        // Empty state
        div()
            .flex_1()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .child("Select a persona to start a conversation")
            .into_any_element()
    }

    fn render_placeholder_view(&self, title: impl Into<SharedString>, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex_1()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                v_flex()
                    .gap_2()
                    .items_center()
                    .child(Label::new(title).text_xl())
                    .child(
                        Label::new("Coming soon...")
                            .text_color(cx.theme().muted_foreground),
                    ),
            )
            .into_any_element()
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(self.nav_bar.clone())
            .child(self.render_main_content(window, cx))
    }
}
