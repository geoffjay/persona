use crate::knowledgebase::{self, KnowledgebaseEntry};
use crate::persona::Persona;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{h_flex, v_flex, ActiveTheme};
use std::path::PathBuf;

use super::knowledgebase_slideout::KnowledgebaseSlideout;
use super::knowledgebase_table::KnowledgebaseTable;

#[derive(Debug, Clone)]
pub struct PersonaTab {
    pub name: String,
    pub kb_path: PathBuf,
}

pub struct KnowledgebaseView {
    personas: Vec<PersonaTab>,
    selected_index: usize,
    entries: Vec<KnowledgebaseEntry>,
    table: Entity<KnowledgebaseTable>,
    slideout: Entity<KnowledgebaseSlideout>,
    slideout_open: bool,
    selected_entry: Option<KnowledgebaseEntry>,
}

impl KnowledgebaseView {
    pub fn new(personas: Vec<Persona>, cx: &mut Context<Self>) -> Self {
        let entity = cx.entity().clone();

        // Filter personas that have knowledgebases
        let persona_tabs: Vec<PersonaTab> = personas
            .into_iter()
            .filter_map(|p| {
                p.knowledgebase_path.map(|kb_path| PersonaTab {
                    name: p.name,
                    kb_path,
                })
            })
            .collect();

        // Create table with selection callback
        let entity_for_table = entity.clone();
        let table = cx.new(|_cx| {
            KnowledgebaseTable::new(vec![], move |_index, entry, window, cx| {
                entity_for_table.update(cx, |this, cx| {
                    this.selected_entry = Some(entry.clone());
                    this.slideout_open = true;

                    if let Ok(file) = knowledgebase::load_file(&entry.file_path) {
                        this.slideout.update(cx, |slideout, cx| {
                            slideout.set_file(Some(file), window, cx);
                            cx.notify();
                        });
                    }
                    cx.notify();
                });
            })
        });

        // Create slideout with close callback
        let entity_for_slideout = entity.clone();
        let slideout = cx.new(|_cx| {
            KnowledgebaseSlideout::new(move |_window, cx| {
                entity_for_slideout.update(cx, |this, cx| {
                    this.slideout_open = false;
                    this.selected_entry = None;
                    this.table.update(cx, |table, cx| {
                        table.set_selected(None);
                        cx.notify();
                    });
                    cx.notify();
                });
            })
        });

        let mut view = Self {
            personas: persona_tabs,
            selected_index: 0,
            entries: vec![],
            table,
            slideout,
            slideout_open: false,
            selected_entry: None,
        };

        // Load initial entries if there are personas
        view.load_entries_for_selected(cx);

        view
    }

    fn load_entries_for_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(persona) = self.personas.get(self.selected_index) {
            let entries = knowledgebase::load_entries(&persona.kb_path);
            self.entries = entries.clone();
            self.table.update(cx, |table, cx| {
                table.set_entries(entries);
                cx.notify();
            });
        } else {
            self.entries = vec![];
            self.table.update(cx, |table, cx| {
                table.set_entries(vec![]);
                cx.notify();
            });
        }
    }

    fn select_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index != self.selected_index && index < self.personas.len() {
            self.selected_index = index;
            // Close slideout when switching tabs
            self.slideout_open = false;
            self.selected_entry = None;
            self.load_entries_for_selected(cx);
            cx.notify();
        }
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut tab_bar = TabBar::new("knowledgebase-tabs")
            .selected_index(self.selected_index)
            .on_click(cx.listener(move |this, index: &usize, _window, cx| {
                this.select_tab(*index, cx);
            }));

        for persona in &self.personas {
            let tab = Tab::new().label(persona.name.clone());
            tab_bar = tab_bar.child(tab);
        }

        h_flex()
            .w_full()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(tab_bar)
    }

    fn render_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.personas.is_empty() {
            div()
                .flex_1()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(cx.theme().muted_foreground)
                .child("No personas with knowledgebases found")
                .into_any_element()
        } else if self.entries.is_empty() {
            div()
                .flex_1()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(cx.theme().muted_foreground)
                .child("No knowledgebase files found")
                .into_any_element()
        } else {
            self.table.clone().into_any_element()
        }
    }
}

impl Render for KnowledgebaseView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_content = v_flex()
            .flex_1()
            .h_full()
            .when(!self.personas.is_empty(), |this| {
                this.child(self.render_tabs(cx))
            })
            .child(self.render_content(cx));

        let mut content = h_flex().flex_1().h_full().child(main_content);

        if self.slideout_open {
            content = content.child(self.slideout.clone());
        }

        content
    }
}
