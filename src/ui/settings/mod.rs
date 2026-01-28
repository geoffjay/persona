mod terminal_settings;

pub use terminal_settings::TerminalSettingsPanel;

use gpui::*;
use gpui_component::list::ListItem;
use gpui_component::{h_flex, label::Label, v_flex, ActiveTheme, Icon, IconName, Sizable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    Terminal,
}

impl SettingsCategory {
    pub fn all() -> &'static [SettingsCategory] {
        &[SettingsCategory::Terminal]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Terminal => "Terminal",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::Terminal => IconName::SquareTerminal,
        }
    }
}

pub struct SettingsView {
    selected_category: SettingsCategory,
    terminal_panel: Entity<TerminalSettingsPanel>,
}

impl SettingsView {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let terminal_panel = cx.new(|_cx| TerminalSettingsPanel::new());

        Self {
            selected_category: SettingsCategory::Terminal,
            terminal_panel,
        }
    }

    fn render_category_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let categories = SettingsCategory::all();

        v_flex()
            .id("settings-categories")
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
                    .child(Label::new("Settings").text_sm()),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .children(
                        categories
                            .iter()
                            .map(|cat| self.render_category_item(*cat, cx)),
                    ),
            )
    }

    fn render_category_item(
        &self,
        category: SettingsCategory,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected_category == category;
        let entity = cx.entity().clone();

        ListItem::new(category.label())
            .py_2()
            .px_3()
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

    fn render_content(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.selected_category {
            SettingsCategory::Terminal => self.terminal_panel.clone().into_any_element(),
        }
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(self.render_category_list(cx))
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .bg(cx.theme().background)
                    .child(self.render_content(cx)),
            )
    }
}
