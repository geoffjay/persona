use crate::persona::Persona;
use crate::ui::persona::conversation::ConversationView;
use crate::ui::persona::list::PersonaList;
use crate::ui::persona::terminal_header_bar::{TerminalHeaderBar, TerminalHeaderBarEvent};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme};
use std::collections::HashMap;

/// Stores a session's conversation view and header bar
struct Session {
    conversation: Entity<ConversationView>,
    header: Entity<TerminalHeaderBar>,
}

pub struct PersonaPanel {
    #[allow(dead_code)]
    personas: Vec<Persona>,
    selected_persona: Option<Persona>,
    persona_list: Entity<PersonaList>,
    /// Active sessions keyed by persona ID
    sessions: HashMap<String, Session>,
    is_expanded: bool,
    _subscriptions: Vec<Subscription>,
}

impl PersonaPanel {
    pub fn new(personas: Vec<Persona>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let entity = cx.entity().clone();

        let persona_list = cx.new(|_cx| {
            let entity = entity.clone();
            PersonaList::new(personas.clone(), move |persona, _window, cx| {
                entity.update(cx, |this, cx| {
                    this.select_persona(persona.clone(), cx);
                });
            })
        });

        Self {
            personas,
            selected_persona: None,
            persona_list,
            sessions: HashMap::new(),
            is_expanded: false,
            _subscriptions: Vec::new(),
        }
    }

    fn select_persona(&mut self, persona: Persona, cx: &mut Context<Self>) {
        self.selected_persona = Some(persona.clone());

        // If this persona has an active session, restore the expanded state from the header
        if let Some(session) = self.sessions.get(&persona.id) {
            let is_expanded = session.header.read(cx).is_expanded();
            self.is_expanded = is_expanded;
        } else {
            // No active session, collapse sidebar
            self.is_expanded = false;
        }

        cx.notify();
    }

    fn start_session(
        &mut self,
        persona: &Persona,
        continue_session: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let persona_clone = persona.clone();
        let conv =
            cx.new(|cx| ConversationView::new(persona_clone.clone(), continue_session, window, cx));

        // Create terminal header bar
        let header =
            cx.new(|_cx| TerminalHeaderBar::new(persona_clone.name.clone(), self.is_expanded));

        // Subscribe to header bar events
        let persona_id_for_toggle = persona.id.clone();
        let persona_id_for_close = persona.id.clone();
        let subscription = cx.subscribe(&header, move |this, _header, event, cx| match event {
            TerminalHeaderBarEvent::ToggleExpanded => {
                this.is_expanded = !this.is_expanded;
                // Update the header bar's expanded state for this session
                if let Some(session) = this.sessions.get(&persona_id_for_toggle) {
                    session.header.update(cx, |h, hcx| {
                        h.set_expanded(this.is_expanded);
                        hcx.notify();
                    });
                }
                cx.notify();
            }
            TerminalHeaderBarEvent::CloseSession => {
                this.close_session(&persona_id_for_close, cx);
            }
        });

        self._subscriptions.push(subscription);

        // Store the session
        let session = Session {
            conversation: conv,
            header,
        };
        self.sessions.insert(persona.id.clone(), session);

        // Update the persona list to show active sessions
        self.sync_active_sessions(cx);

        cx.notify();
    }

    fn close_session(&mut self, persona_id: &str, cx: &mut Context<Self>) {
        // Remove the session
        self.sessions.remove(persona_id);

        // Collapse the sidebar when closing
        self.is_expanded = false;

        // Update the persona list to remove the active session badge
        self.sync_active_sessions(cx);

        cx.notify();
    }

    fn sync_active_sessions(&self, cx: &mut Context<Self>) {
        let active_ids: std::collections::HashSet<String> =
            self.sessions.keys().cloned().collect();
        let persona_list = self.persona_list.clone();
        cx.defer(move |cx| {
            persona_list.update(cx, |list, lcx| {
                list.set_active_sessions(active_ids);
                lcx.notify();
            });
        });
    }

    fn render_session_buttons(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        let persona = self.selected_persona.clone().unwrap();

        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .child(Label::new(format!("Start session with {}", persona.name)))
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        Button::new("new-session")
                            .label("Start New Session")
                            .primary()
                            .on_click({
                                let entity = entity.clone();
                                let persona = persona.clone();
                                move |_, window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.start_session(&persona, false, window, cx);
                                    });
                                }
                            }),
                    )
                    .child(
                        Button::new("continue-session")
                            .label("Continue Session")
                            .ghost()
                            .on_click({
                                let entity = entity.clone();
                                let persona = persona.clone();
                                move |_, window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.start_session(&persona, true, window, cx);
                                    });
                                }
                            }),
                    ),
            )
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .child("Select a persona to start a conversation")
    }

    fn render_conversation_area(&self, cx: &mut Context<Self>) -> AnyElement {
        let Some(persona) = &self.selected_persona else {
            return self.render_empty_state(cx).into_any_element();
        };

        // Check if this persona has an active session
        let Some(session) = self.sessions.get(&persona.id) else {
            return self.render_session_buttons(cx).into_any_element();
        };

        // Active session - show header bar + conversation
        v_flex()
            .flex_1()
            .size_full()
            .child(session.header.clone())
            .child(session.conversation.clone())
            .into_any_element()
    }
}

impl Render for PersonaPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar_width = if self.is_expanded { 0. } else { 300. };

        h_flex()
            .size_full()
            .child(
                div()
                    .w(px(sidebar_width))
                    .h_full()
                    .overflow_hidden()
                    .when(!self.is_expanded, |this| {
                        this.child(self.persona_list.clone())
                    }),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .child(self.render_conversation_area(cx)),
            )
    }
}
