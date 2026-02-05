use crate::persona::Persona;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationView {
    #[default]
    Personas,
    Memory,
    Settings,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_view: NavigationView,
    pub personas: Vec<Persona>,
}

impl AppState {
    pub fn new(personas: Vec<Persona>) -> Self {
        Self {
            current_view: NavigationView::Personas,
            personas,
        }
    }
}
