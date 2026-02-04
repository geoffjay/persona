use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct PersonaFrontmatter {
    pub persona_id: String,
    pub avatar_url: Option<String>,
    pub knowledgebase_dir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Persona {
    pub id: String,
    pub name: String,
    pub file_path: PathBuf,
    pub avatar_url: Option<String>,
    pub knowledgebase_path: Option<PathBuf>,
}

impl Persona {
    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(&path)?;

        // Parse YAML frontmatter between --- delimiters
        let frontmatter = Self::extract_frontmatter(&content)?;
        let meta: PersonaFrontmatter = serde_yaml::from_str(&frontmatter)?;

        // Extract name from first # heading or use file name
        let name = Self::extract_name(&content).unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .replace('-', " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        });

        // Resolve knowledgebase path relative to persona file's directory
        let knowledgebase_path = meta.knowledgebase_dir.and_then(|kb_path| {
            let kb_pathbuf = PathBuf::from(&kb_path);
            let resolved = if kb_pathbuf.is_absolute() {
                kb_pathbuf
            } else {
                path.parent()?.join(kb_pathbuf)
            };
            if resolved.exists() && resolved.is_dir() {
                Some(resolved)
            } else {
                eprintln!(
                    "Warning: Knowledgebase directory does not exist: {:?}",
                    resolved
                );
                None
            }
        });

        Ok(Self {
            id: meta.persona_id,
            name,
            file_path: path,
            avatar_url: meta.avatar_url,
            knowledgebase_path,
        })
    }

    fn extract_frontmatter(content: &str) -> anyhow::Result<String> {
        let content = content.trim_start();
        if !content.starts_with("---") {
            anyhow::bail!("No YAML frontmatter found");
        }

        let after_first = &content[3..];
        let end = after_first
            .find("---")
            .ok_or_else(|| anyhow::anyhow!("Unclosed frontmatter"))?;

        Ok(after_first[..end].to_string())
    }

    fn extract_name(content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return Some(trimmed[2..].trim().to_string());
            }
        }
        None
    }
}
