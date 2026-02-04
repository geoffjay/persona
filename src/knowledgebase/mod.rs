use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct KnowledgebaseEntry {
    pub file_path: PathBuf,
    pub name: String,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct KnowledgebaseFile {
    pub entry: KnowledgebaseEntry,
    pub content: String,
}

pub fn load_entries(kb_path: &Path) -> Vec<KnowledgebaseEntry> {
    let mut entries = Vec::new();

    let Ok(read_dir) = std::fs::read_dir(kb_path) else {
        return entries;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            if let Some(kb_entry) = create_entry(&path) {
                entries.push(kb_entry);
            }
        }
    }

    // Sort by modified date, newest first
    entries.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    entries
}

fn create_entry(path: &Path) -> Option<KnowledgebaseEntry> {
    let metadata = std::fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let modified_at: DateTime<Utc> = modified.into();

    // Try to extract name from first # heading, fall back to filename
    let name = extract_name_from_file(path).unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.replace('-', " ").replace('_', " "))
            .unwrap_or_else(|| "Unknown".to_string())
    });

    Some(KnowledgebaseEntry {
        file_path: path.to_path_buf(),
        name,
        modified_at,
    })
}

fn extract_name_from_file(path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return Some(trimmed[2..].trim().to_string());
        }
    }
    None
}

pub fn load_file(path: &Path) -> anyhow::Result<KnowledgebaseFile> {
    let content = std::fs::read_to_string(path)?;
    let entry = create_entry(path).ok_or_else(|| anyhow::anyhow!("Failed to create entry"))?;

    Ok(KnowledgebaseFile { entry, content })
}

pub fn save_file(path: &Path, content: &str) -> anyhow::Result<()> {
    std::fs::write(path, content)?;
    Ok(())
}
