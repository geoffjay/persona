use crate::persona::Persona;
use gpui::*;
use gpui_component::avatar::Avatar;
use gpui_component::badge::Badge;
use gpui_component::list::ListItem;
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme, Sizable};
use std::collections::HashSet;

pub struct PersonaList {
    pub personas: Vec<Persona>,
    pub selected_index: Option<usize>,
    pub active_session_ids: HashSet<String>,
    pub on_select: Box<dyn Fn(&Persona, &mut Window, &mut Context<Self>) + 'static>,
}

impl PersonaList {
    pub fn new<F>(personas: Vec<Persona>, on_select: F) -> Self
    where
        F: Fn(&Persona, &mut Window, &mut Context<Self>) + 'static,
    {
        Self {
            personas,
            selected_index: None,
            active_session_ids: HashSet::new(),
            on_select: Box::new(on_select),
        }
    }

    pub fn set_active_sessions(&mut self, ids: HashSet<String>) {
        self.active_session_ids = ids;
    }

    fn render_avatar(&self, persona: &Persona, cx: &mut Context<Self>) -> impl IntoElement {
        let mut avatar = Avatar::new().name(persona.name.clone()).small();
        if let Some(url) = &persona.avatar_url {
            avatar = avatar.src(url.clone());
        }

        let has_active_session = self.active_session_ids.contains(&persona.id);

        if has_active_session {
            Badge::new()
                .dot()
                .color(cx.theme().success)
                .small()
                .child(avatar)
        } else {
            Badge::new().child(avatar)
        }
    }

    fn render_persona_item(
        &self,
        index: usize,
        persona: &Persona,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected_index == Some(index);
        let entity = cx.entity().clone();
        let persona_clone = persona.clone();

        ListItem::new(("persona", index))
            .py_2()
            .px_3()
            .h(px(48.))
            .selected(is_selected)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(self.render_avatar(persona, cx))
                    .child(Label::new(persona.name.clone())),
            )
            .on_click(move |_, window, cx| {
                entity.update(cx, |this, inner_cx| {
                    this.selected_index = Some(index);
                    (this.on_select)(&persona_clone, window, inner_cx);
                });
            })
    }
}

impl Render for PersonaList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items: Vec<_> = self
            .personas
            .iter()
            .enumerate()
            .map(|(i, p)| self.render_persona_item(i, p, cx))
            .collect();

        v_flex()
            .id("persona-list")
            .h_full()
            .w(px(300.))
            .items_start()
            .border_r_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().sidebar)
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(Label::new("Personas").text_sm()),
            )
            .child(div().w_full().flex_1().overflow_hidden().children(items))
    }
}
