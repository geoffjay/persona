use crate::persona::Persona;
use crate::ui::persona::conversation::ConversationView;
use crate::ui::persona::list::PersonaList;
use crate::ui::persona::terminal_header_bar::{TerminalHeaderBar, TerminalHeaderBarEvent};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme};

pub struct PersonaPanel {
    #[allow(dead_code)]
    personas: Vec<Persona>,
    selected_persona: Option<Persona>,
    persona_list: Entity<PersonaList>,
    conversation: Option<Entity<ConversationView>>,
    terminal_header: Option<Entity<TerminalHeaderBar>>,
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
                    // Just select, don't start session
                    this.selected_persona = Some(persona.clone());
                    this.conversation = None;
                    this.terminal_header = None;
                    this.is_expanded = false;
                    cx.notify();
                });
            })
        });

        Self {
            personas,
            selected_persona: None,
            persona_list,
            conversation: None,
            terminal_header: None,
            is_expanded: false,
            _subscriptions: Vec::new(),
        }
    }

    fn start_session(
        &mut self,
        persona: &Persona,
        continue_session: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let persona_clone = persona.clone();
        let conv = cx.new(|cx| ConversationView::new(persona_clone.clone(), continue_session, window, cx));
        self.conversation = Some(conv);

        // Create terminal header bar
        let header = cx.new(|_cx| TerminalHeaderBar::new(persona_clone.name.clone(), self.is_expanded));

        // Subscribe to header bar events
        let subscription = cx.subscribe(&header, |this, _header, event, cx| match event {
            TerminalHeaderBarEvent::ToggleExpanded => {
                this.is_expanded = !this.is_expanded;
                // Update the header bar's expanded state
                if let Some(header) = &this.terminal_header {
                    header.update(cx, |h, hcx| {
                        h.set_expanded(this.is_expanded);
                        hcx.notify();
                    });
                }
                cx.notify();
            }
        });

        self._subscriptions.push(subscription);
        self.terminal_header = Some(header);

        cx.notify();
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
        if self.selected_persona.is_none() {
            return self.render_empty_state(cx).into_any_element();
        }

        if self.conversation.is_none() {
            return self.render_session_buttons(cx).into_any_element();
        }

        // Active session - show header bar + conversation
        let mut content = v_flex().flex_1().size_full();

        if let Some(header) = &self.terminal_header {
            content = content.child(header.clone());
        }

        if let Some(conv) = &self.conversation {
            content = content.child(conv.clone());
        }

        content.into_any_element()
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
