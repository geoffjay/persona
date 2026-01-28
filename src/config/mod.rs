use crate::persona::Persona;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub personas_dir: PathBuf,
    pub berry_server_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        // Default to ~/.config/persona-ui/personas or a local personas directory
        let personas_dir = dirs::config_dir()
            .map(|p| p.join("persona-ui").join("personas"))
            .unwrap_or_else(|| PathBuf::from("personas"));

        let berry_server_url = std::env::var("BERRY_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:4114".to_string());

        Self {
            personas_dir,
            berry_server_url,
        }
    }
}

impl AppConfig {
    pub fn with_personas_dir(mut self, dir: PathBuf) -> Self {
        self.personas_dir = dir;
        self
    }

    pub fn with_berry_server_url(mut self, url: String) -> Self {
        self.berry_server_url = url;
        self
    }

    pub fn load_personas(&self) -> Vec<Persona> {
        let mut personas = Vec::new();

        if !self.personas_dir.exists() {
            eprintln!("Personas directory not found: {:?}", self.personas_dir);
            return personas;
        }

        let entries = match std::fs::read_dir(&self.personas_dir) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Failed to read personas directory: {}", err);
                return personas;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match Persona::from_file(path.clone()) {
                    Ok(persona) => personas.push(persona),
                    Err(err) => eprintln!("Failed to load persona from {:?}: {}", path, err),
                }
            }
        }

        // Sort by name
        personas.sort_by(|a, b| a.name.cmp(&b.name));
        personas
    }
}
