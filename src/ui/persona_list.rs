use crate::persona::Persona;
use gpui::*;
use gpui_component::list::ListItem;
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme, Icon, IconName, Sizable};

pub struct PersonaList {
    pub personas: Vec<Persona>,
    pub selected_index: Option<usize>,
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
            on_select: Box::new(on_select),
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
            .selected(is_selected)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .size_8()
                            .rounded_full()
                            .bg(cx.theme().accent)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(IconName::User).small().text_color(cx.theme().accent_foreground)),
                    )
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
        let personas = self.personas.clone();

        v_flex()
            .id("persona-list")
            .h_full()
            .w(px(200.))
            .border_r_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().sidebar)
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(Label::new("Personas").text_sm()),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .children(
                        personas
                            .iter()
                            .enumerate()
                            .map(|(i, p)| self.render_persona_item(i, p, cx)),
                    ),
            )
    }
}
