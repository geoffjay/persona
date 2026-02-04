use crate::knowledgebase::KnowledgebaseEntry;
use gpui::*;
use gpui_component::{h_flex, list::ListItem, v_flex, ActiveTheme};

pub struct KnowledgebaseTable {
    entries: Vec<KnowledgebaseEntry>,
    selected_index: Option<usize>,
    on_select: Box<dyn Fn(usize, &KnowledgebaseEntry, &mut Window, &mut App) + 'static>,
}

impl KnowledgebaseTable {
    pub fn new<F>(entries: Vec<KnowledgebaseEntry>, on_select: F) -> Self
    where
        F: Fn(usize, &KnowledgebaseEntry, &mut Window, &mut App) + 'static,
    {
        Self {
            entries,
            selected_index: None,
            on_select: Box::new(on_select),
        }
    }

    pub fn set_entries(&mut self, entries: Vec<KnowledgebaseEntry>) {
        self.entries = entries;
        self.selected_index = None;
    }

    pub fn set_selected(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().muted)
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Name"),
            )
            .child(
                div()
                    .w(px(160.))
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Modified"),
            )
    }

    fn render_row(
        &self,
        index: usize,
        entry: &KnowledgebaseEntry,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected_index == Some(index);
        let entry_clone = entry.clone();
        let entity = cx.entity().clone();

        let modified = entry.modified_at.format("%Y-%m-%d %H:%M").to_string();

        ListItem::new(("kb-row", index))
            .px_3()
            .py_1()
            .selected(is_selected)
            .on_click(move |_, window, cx| {
                entity.update(cx, |this, cx| {
                    this.selected_index = Some(index);
                    (this.on_select)(index, &entry_clone, window, cx);
                    cx.notify();
                });
            })
            .child(
                h_flex()
                    .w_full()
                    .gap_0()
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .overflow_hidden()
                            .child(entry.name.clone()),
                    )
                    .child(
                        div()
                            .w(px(160.))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(modified),
                    ),
            )
    }
}

impl Render for KnowledgebaseTable {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows: Vec<_> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| self.render_row(i, e, cx))
            .collect();

        v_flex()
            .id("kb-table")
            .w_full()
            .flex_1()
            .overflow_hidden()
            .child(self.render_header(cx))
            .child(
                v_flex()
                    .id("kb-table-rows")
                    .w_full()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}
