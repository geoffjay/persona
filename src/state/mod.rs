use crate::persona::Persona;
use gpui::SharedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationView {
    Personas,
    Memory,
    Settings,
}

impl Default for NavigationView {
    fn default() -> Self {
        Self::Personas
    }
}

#[derive(Debug, Clone)]
pub struct ConversationTab {
    pub persona_id: String,
    pub persona_name: SharedString,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_view: NavigationView,
    pub personas: Vec<Persona>,
    pub open_tabs: Vec<ConversationTab>,
    pub active_tab_index: Option<usize>,
}

impl AppState {
    pub fn new(personas: Vec<Persona>) -> Self {
        Self {
            current_view: NavigationView::Personas,
            personas,
            open_tabs: Vec::new(),
            active_tab_index: None,
        }
    }

    pub fn open_conversation(&mut self, persona: &Persona) {
        // Check if tab already exists
        if let Some(index) = self
            .open_tabs
            .iter()
            .position(|t| t.persona_id == persona.id)
        {
            self.active_tab_index = Some(index);
            return;
        }

        // Create new tab
        let tab = ConversationTab {
            persona_id: persona.id.clone(),
            persona_name: persona.name.clone().into(),
        };
        self.open_tabs.push(tab);
        self.active_tab_index = Some(self.open_tabs.len() - 1);
    }

    pub fn close_tab(&mut self, index: usize) {
        if index >= self.open_tabs.len() {
            return;
        }

        self.open_tabs.remove(index);

        // Adjust active tab index
        if self.open_tabs.is_empty() {
            self.active_tab_index = None;
        } else if let Some(active) = self.active_tab_index {
            if active >= self.open_tabs.len() {
                self.active_tab_index = Some(self.open_tabs.len() - 1);
            } else if active > index {
                self.active_tab_index = Some(active - 1);
            }
        }
    }

    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.open_tabs.len() {
            self.active_tab_index = Some(index);
        }
    }

    pub fn get_persona_by_id(&self, id: &str) -> Option<&Persona> {
        self.personas.iter().find(|p| p.id == id)
    }
}
