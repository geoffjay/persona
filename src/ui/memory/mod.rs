mod knowledgebase_slideout;
mod knowledgebase_table;
mod slideout;
mod table;

use crate::knowledgebase::{self, KnowledgebaseEntry};
use crate::memory::{BerryClient, BerryError, Memory, SearchRequest};
use crate::persona::Persona;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    h_flex,
    input::{Input, InputEvent, InputState},
    label::Label,
    list::ListItem,
    v_flex, ActiveTheme, Icon, IconName, Sizable,
};
use gpui_tokio_bridge::{JoinError, Tokio};
use knowledgebase_slideout::KnowledgebaseSlideout;
use knowledgebase_table::KnowledgebaseTable;
use slideout::MemorySlideout;
use std::path::PathBuf;
use table::MemoryTable;

#[derive(Debug, Clone, PartialEq)]
pub struct PersonaKnowledgebase {
    pub persona_id: String,
    pub persona_name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryCategory {
    Berry,
    Knowledgebase(PersonaKnowledgebase),
}

impl MemoryCategory {
    pub fn label(&self) -> String {
        match self {
            Self::Berry => "Berry".to_string(),
            Self::Knowledgebase(kb) => kb.persona_name.clone(),
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::Berry => IconName::Bot,
            Self::Knowledgebase(_) => IconName::BookOpen,
        }
    }

    pub fn id(&self) -> String {
        match self {
            Self::Berry => "berry".to_string(),
            Self::Knowledgebase(kb) => format!("kb-{}", kb.persona_id),
        }
    }
}

fn build_categories(personas: &[Persona]) -> Vec<MemoryCategory> {
    let mut categories = vec![MemoryCategory::Berry];

    for persona in personas {
        if let Some(kb_path) = &persona.knowledgebase_path {
            categories.push(MemoryCategory::Knowledgebase(PersonaKnowledgebase {
                persona_id: persona.id.clone(),
                persona_name: persona.name.clone(),
                path: kb_path.clone(),
            }));
        }
    }

    categories
}

#[derive(Debug, Clone)]
pub enum LoadingState<T> {
    Idle,
    Loading,
    Loaded(T),
    Error(String),
}

pub struct MemoryView {
    categories: Vec<MemoryCategory>,
    selected_category: MemoryCategory,
    client: BerryClient,
    memories: LoadingState<Vec<Memory>>,
    selected_memory: Option<Memory>,
    search_input: Option<Entity<InputState>>,
    table: Entity<MemoryTable>,
    slideout: Entity<MemorySlideout>,
    slideout_open: bool,
    // Knowledgebase state
    kb_entries: Vec<KnowledgebaseEntry>,
    kb_table: Entity<KnowledgebaseTable>,
    kb_slideout: Entity<KnowledgebaseSlideout>,
    kb_slideout_open: bool,
    selected_kb_entry: Option<KnowledgebaseEntry>,
}

impl MemoryView {
    pub fn new(
        berry_server_url: String,
        personas: Vec<Persona>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let client = BerryClient::new(berry_server_url);
        let entity = cx.entity().clone();
        let categories = build_categories(&personas);

        // Create Berry memory table with selection callback
        let entity_for_table = entity.clone();
        let table = cx.new(|_cx| {
            MemoryTable::new(vec![], move |_index, memory, _window, cx| {
                entity_for_table.update(cx, |this, cx| {
                    this.selected_memory = Some(memory.clone());
                    this.slideout_open = true;
                    this.slideout.update(cx, |slideout, cx| {
                        slideout.set_memory(Some(memory.clone()));
                        cx.notify();
                    });
                    cx.notify();
                });
            })
        });

        // Create Berry slideout with close callback
        let entity_for_slideout = entity.clone();
        let slideout = cx.new(|_cx| {
            MemorySlideout::new(move |_window, cx| {
                entity_for_slideout.update(cx, |this, cx| {
                    this.slideout_open = false;
                    this.selected_memory = None;
                    this.table.update(cx, |table, cx| {
                        table.set_selected(None);
                        cx.notify();
                    });
                    cx.notify();
                });
            })
        });

        // Create knowledgebase table with selection callback
        let entity_for_kb_table = entity.clone();
        let kb_table = cx.new(|_cx| {
            KnowledgebaseTable::new(vec![], move |_index, entry, window, cx| {
                entity_for_kb_table.update(cx, |this, cx| {
                    this.selected_kb_entry = Some(entry.clone());
                    this.kb_slideout_open = true;

                    // Load file content
                    if let Ok(file) = knowledgebase::load_file(&entry.file_path) {
                        this.kb_slideout.update(cx, |slideout, cx| {
                            slideout.set_file(Some(file), window, cx);
                            cx.notify();
                        });
                    }
                    cx.notify();
                });
            })
        });

        // Create knowledgebase slideout with close callback
        let entity_for_kb_slideout = entity.clone();
        let kb_slideout = cx.new(|_cx| {
            KnowledgebaseSlideout::new(move |_window, cx| {
                entity_for_kb_slideout.update(cx, |this, cx| {
                    this.kb_slideout_open = false;
                    this.selected_kb_entry = None;
                    this.kb_table.update(cx, |table, cx| {
                        table.set_selected(None);
                        cx.notify();
                    });
                    cx.notify();
                });
            })
        });

        // Create search input
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search memories..."));

        // Subscribe to input changes
        cx.subscribe(
            &search_input,
            move |this, _input, event: &InputEvent, cx| {
                if let InputEvent::PressEnter { secondary: _ } = event {
                    this.fetch_memories(cx);
                }
            },
        )
        .detach();

        let mut view = Self {
            categories,
            selected_category: MemoryCategory::Berry,
            client,
            memories: LoadingState::Idle,
            selected_memory: None,
            search_input: Some(search_input),
            table,
            slideout,
            slideout_open: false,
            kb_entries: vec![],
            kb_table,
            kb_slideout,
            kb_slideout_open: false,
            selected_kb_entry: None,
        };

        // Fetch initial memories
        view.fetch_memories(cx);

        view
    }

    fn fetch_memories(&mut self, cx: &mut Context<Self>) {
        self.memories = LoadingState::Loading;
        cx.notify();

        let entity = cx.entity().clone();
        let table = self.table.clone();
        let client = self.client.clone();
        let query = self
            .search_input
            .as_ref()
            .map(|input| input.read(cx).text().to_string())
            .unwrap_or_default();

        let task = Tokio::spawn(cx, async move {
            client
                .search(SearchRequest {
                    query,
                    as_actor: "persona-ui".to_string(),
                    limit: Some(25),
                    ..Default::default()
                })
                .await
        });

        cx.spawn(async move |_this, cx| {
            let result: Result<Result<Vec<Memory>, BerryError>, JoinError> = task.await;
            cx.update(|cx: &mut App| {
                entity.update(cx, |this, cx| {
                    match result {
                        Ok(Ok(memories)) => {
                            this.memories = LoadingState::Loaded(memories.clone());
                            table.update(cx, |t, cx| {
                                t.set_memories(memories);
                                cx.notify();
                            });
                        }
                        Ok(Err(e)) => {
                            this.memories = LoadingState::Error(e.to_string());
                        }
                        Err(e) => {
                            this.memories = LoadingState::Error(e.to_string());
                        }
                    }
                    cx.notify();
                });
            })
        })
        .detach();
    }

    fn render_search_bar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_input = self.search_input.clone();

        h_flex()
            .w_full()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(cx.theme().border)
            .when_some(search_input, |this, input| {
                this.child(
                    Input::new(&input)
                        .w_full()
                        .cleanable(true)
                        .appearance(false),
                )
            })
    }

    fn render_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.memories {
            LoadingState::Idle | LoadingState::Loading => div()
                .flex_1()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(cx.theme().muted_foreground)
                .child("Loading memories...")
                .into_any_element(),

            LoadingState::Error(err) => div()
                .flex_1()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(cx.theme().danger)
                .child(format!("Error: {}", err))
                .into_any_element(),

            LoadingState::Loaded(memories) => {
                if memories.is_empty() {
                    div()
                        .flex_1()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(cx.theme().muted_foreground)
                        .child("No memories found")
                        .into_any_element()
                } else {
                    self.table.clone().into_any_element()
                }
            }
        }
    }

    fn render_category_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        let selected_category = self.selected_category.clone();
        let categories: Vec<_> = self.categories.clone();

        v_flex()
            .id("memory-categories")
            .h_full()
            .w(px(300.))
            .border_r_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().sidebar)
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(Label::new("Memory").text_sm()),
            )
            .child(
                div().flex_1().overflow_hidden().children(
                    categories
                        .into_iter()
                        .enumerate()
                        .map(|(index, category)| {
                            let is_selected = selected_category == category;
                            let entity = entity.clone();
                            let category_for_click = category.clone();

                            ListItem::new(("memory-cat", index))
                                .py_2()
                                .px_3()
                                .h(px(48.))
                                .selected(is_selected)
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .items_center()
                                        .child(Icon::new(category.icon()).small())
                                        .child(Label::new(category.label())),
                                )
                                .on_click(move |_, _window, cx| {
                                    let cat = category_for_click.clone();
                                    entity.update(cx, |this, cx| {
                                        this.select_category(cat, cx);
                                    });
                                })
                        })
                        .collect::<Vec<_>>(),
                ),
            )
    }

    fn select_category(&mut self, category: MemoryCategory, cx: &mut Context<Self>) {
        self.selected_category = category.clone();

        // Load knowledgebase entries if selecting a knowledgebase category
        if let MemoryCategory::Knowledgebase(kb) = &category {
            let entries = knowledgebase::load_entries(&kb.path);
            self.kb_entries = entries.clone();
            self.kb_table.update(cx, |table, cx| {
                table.set_entries(entries);
                cx.notify();
            });
        }

        cx.notify();
    }

    fn render_main_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match &self.selected_category {
            MemoryCategory::Berry => {
                let mut content = h_flex().flex_1().h_full().child(
                    v_flex()
                        .flex_1()
                        .h_full()
                        .child(self.render_search_bar(window, cx))
                        .child(self.render_content(window, cx)),
                );

                if self.slideout_open {
                    content = content.child(self.slideout.clone());
                }

                content.into_any_element()
            }
            MemoryCategory::Knowledgebase(_kb) => {
                let mut content = h_flex()
                    .flex_1()
                    .h_full()
                    .child(v_flex().flex_1().h_full().child(self.render_kb_content(cx)));

                if self.kb_slideout_open {
                    content = content.child(self.kb_slideout.clone());
                }

                content.into_any_element()
            }
        }
    }

    fn render_kb_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.kb_entries.is_empty() {
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
            self.kb_table.clone().into_any_element()
        }
    }
}

impl Render for MemoryView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(self.render_category_list(cx))
            .child(self.render_main_content(window, cx))
    }
}
