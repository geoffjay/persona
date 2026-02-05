use crate::config::AppConfig;
use crate::state::{AppState, NavigationView};
use crate::ui::{FooterBar, HeaderBar, MemoryView, NavigationBar, PersonaPanel, SettingsView};
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme};

pub struct App {
    state: AppState,
    header_bar: Entity<HeaderBar>,
    footer_bar: Entity<FooterBar>,
    nav_bar: Entity<NavigationBar>,
    persona_panel: Entity<PersonaPanel>,
    settings_view: Entity<SettingsView>,
    memory_view: Entity<MemoryView>,
    _subscriptions: Vec<Subscription>,
}

impl App {
    pub fn new(config: AppConfig, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let personas = config.load_personas();
        let state = AppState::new(personas.clone());

        let view = cx.entity().clone();

        let header_bar = cx.new(|_cx| HeaderBar::new(window, _cx));
        let footer_bar = cx.new(|_cx| FooterBar::new(window, _cx));

        let nav_bar = cx.new(|_cx| {
            let view = view.clone();
            NavigationBar::new(state.current_view, move |nav_view, _window, cx| {
                view.update(cx, |this, cx| {
                    this.state.current_view = nav_view;
                    this.sync_nav_bar(cx);
                    cx.notify();
                });
            })
        });

        let personas_for_memory = personas.clone();
        let persona_panel = cx.new(|cx| PersonaPanel::new(personas, window, cx));

        let settings_view = cx.new(|cx| SettingsView::new(window, cx));

        let memory_view = cx.new(|cx| {
            MemoryView::new(
                config.berry_server_url().to_string(),
                personas_for_memory,
                window,
                cx,
            )
        });

        let _subscriptions = vec![];

        Self {
            state,
            header_bar,
            footer_bar,
            nav_bar,
            persona_panel,
            settings_view,
            memory_view,
            _subscriptions,
        }
    }

    fn sync_nav_bar(&self, cx: &mut Context<Self>) {
        let nav_bar = self.nav_bar.clone();
        let current_view = self.state.current_view;
        cx.defer(move |cx| {
            nav_bar.update(cx, |nav, inner_cx| {
                nav.current_view = current_view;
                inner_cx.notify();
            });
        });
    }

    /// Shutdown the application by closing all open sessions
    pub fn shutdown(&mut self, cx: &mut Context<Self>) {
        self.persona_panel.update(cx, |panel, pcx| {
            panel.close_all_sessions(pcx);
        });
    }

    fn render_main_content(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match self.state.current_view {
            NavigationView::Personas => self.render_personas_view(),
            NavigationView::Memory => self.memory_view.clone().into_any_element(),
            NavigationView::Settings => self.settings_view.clone().into_any_element(),
        }
    }

    fn render_personas_view(&self) -> AnyElement {
        self.persona_panel.clone().into_any_element()
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(self.header_bar.clone())
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(self.nav_bar.clone())
                    .child(self.render_main_content(window, cx)),
            )
            .child(self.footer_bar.clone())
    }
}
