mod knowledgebase_slideout;
mod knowledgebase_table;
mod knowledgebase_view;
mod slideout;
mod table;

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
use knowledgebase_view::KnowledgebaseView;
use slideout::MemorySlideout;
use table::MemoryTable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryCategory {
    Berry,
    Knowledgebase,
}

impl MemoryCategory {
    pub fn all() -> &'static [MemoryCategory] {
        &[MemoryCategory::Berry, MemoryCategory::Knowledgebase]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Berry => "Berry",
            Self::Knowledgebase => "Knowledgebase",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::Berry => IconName::Bot,
            Self::Knowledgebase => IconName::BookOpen,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoadingState<T> {
    Idle,
    Loading,
    Loaded(T),
    Error(String),
}

pub struct MemoryView {
    selected_category: MemoryCategory,
    client: BerryClient,
    memories: LoadingState<Vec<Memory>>,
    selected_memory: Option<Memory>,
    search_input: Option<Entity<InputState>>,
    table: Entity<MemoryTable>,
    slideout: Entity<MemorySlideout>,
    slideout_open: bool,
    // Knowledgebase view
    kb_view: Entity<KnowledgebaseView>,
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

        // Create knowledgebase view with all personas
        let kb_view = cx.new(|cx| KnowledgebaseView::new(personas, cx));

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
            selected_category: MemoryCategory::Berry,
            client,
            memories: LoadingState::Idle,
            selected_memory: None,
            search_input: Some(search_input),
            table,
            slideout,
            slideout_open: false,
            kb_view,
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

    fn render_berry_content(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
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
        let categories = MemoryCategory::all();

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
                        .iter()
                        .map(|cat| self.render_category_item(*cat, cx)),
                ),
            )
    }

    fn render_category_item(
        &self,
        category: MemoryCategory,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected_category == category;
        let entity = cx.entity().clone();

        ListItem::new(category.label())
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
                entity.update(cx, |this, cx| {
                    this.selected_category = category;
                    cx.notify();
                });
            })
    }

    fn render_main_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.selected_category {
            MemoryCategory::Berry => {
                let mut content = h_flex().flex_1().h_full().child(
                    v_flex()
                        .flex_1()
                        .h_full()
                        .child(self.render_search_bar(window, cx))
                        .child(self.render_berry_content(window, cx)),
                );

                if self.slideout_open {
                    content = content.child(self.slideout.clone());
                }

                content.into_any_element()
            }
            MemoryCategory::Knowledgebase => self.kb_view.clone().into_any_element(),
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
