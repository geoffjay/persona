use std::path::PathBuf;

/// Returns the application data directory.
///
/// On macOS: `~/Library/Application Support/persona`
/// On Linux: `~/.local/share/persona`
/// On Windows: `C:\Users\<user>\AppData\Roaming\persona`
pub fn data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|p| p.join("persona"))
}

/// Returns true if running in development mode.
///
/// Development mode is detected by:
/// 1. PERSONA_DEV=1 environment variable
/// 2. Cargo.toml exists in current directory (running from source)
pub fn is_dev_mode() -> bool {
    if std::env::var("PERSONA_DEV").is_ok() {
        return true;
    }

    // Check if we're running from the source directory
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("Cargo.toml").exists() && cwd.join(".opencode").exists() {
            return true;
        }
    }

    false
}

/// Returns the working directory for terminals.
///
/// In dev mode: current working directory (project root)
/// In production: data directory (~/Library/Application Support/persona)
pub fn working_dir() -> PathBuf {
    if is_dev_mode() {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    } else {
        data_dir().unwrap_or_else(|| PathBuf::from("."))
    }
}

/// Returns the path to bundled resources.
///
/// For macOS app bundles: `<app>/Contents/Resources`
/// For CLI installed via Homebrew: `<prefix>/share/persona`
/// Returns None if no bundled resources are found.
pub fn bundled_resources_dir() -> Option<PathBuf> {
    // Check for macOS app bundle first
    if let Some(path) = macos_bundle_resources() {
        if path.exists() {
            return Some(path);
        }
    }

    // Check for Homebrew share directory
    if let Some(path) = homebrew_share_dir() {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Get the Resources directory for a macOS app bundle.
fn macos_bundle_resources() -> Option<PathBuf> {
    // Get the executable path
    let exe = std::env::current_exe().ok()?;
    // Navigate: /path/to/Persona.app/Contents/MacOS/persona -> /path/to/Persona.app/Contents/Resources
    let contents = exe.parent()?.parent()?;
    Some(contents.join("Resources"))
}

/// Get the Homebrew share directory.
fn homebrew_share_dir() -> Option<PathBuf> {
    // Check common Homebrew locations
    let candidates = [
        PathBuf::from("/opt/homebrew/share/persona"), // Apple Silicon
        PathBuf::from("/usr/local/share/persona"),    // Intel Mac
        PathBuf::from("/home/linuxbrew/.linuxbrew/share/persona"), // Linux
    ];

    for path in candidates {
        if path.exists() {
            return Some(path);
        }
    }

    // Try to get from HOMEBREW_PREFIX environment variable
    if let Ok(prefix) = std::env::var("HOMEBREW_PREFIX") {
        let path = PathBuf::from(prefix).join("share/persona");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Ensure the data directory exists and contains necessary files.
///
/// On first run, copies bundled resources to the data directory.
/// Returns the data directory path, or None if it couldn't be created.
pub fn ensure_data_dir() -> Option<PathBuf> {
    let data = data_dir()?;

    // Create data directory if it doesn't exist
    if !data.exists() {
        std::fs::create_dir_all(&data).ok()?;
    }

    // Check if we need to bootstrap from bundled resources
    let opencode_dir = data.join(".opencode");
    let personas_dir = data.join("personas");

    let needs_bootstrap = !opencode_dir.exists() || !personas_dir.exists();

    if needs_bootstrap {
        if let Some(bundle) = bundled_resources_dir() {
            bootstrap_from_bundle(&bundle, &data);
        }
    }

    Some(data)
}

/// Copy bundled resources to the data directory.
fn bootstrap_from_bundle(bundle: &PathBuf, data: &PathBuf) {
    // Copy .opencode directory (only specific files)
    let bundle_opencode = bundle.join(".opencode");
    let data_opencode = data.join(".opencode");

    if bundle_opencode.exists() && !data_opencode.exists() {
        if let Err(e) = copy_opencode_dir(&bundle_opencode, &data_opencode) {
            eprintln!("Warning: Failed to copy .opencode directory: {}", e);
        }
    }

    // Copy personas directory
    let bundle_personas = bundle.join("personas");
    let data_personas = data.join("personas");

    if bundle_personas.exists() && !data_personas.exists() {
        if let Err(e) = copy_dir_recursive(&bundle_personas, &data_personas) {
            eprintln!("Warning: Failed to copy personas directory: {}", e);
        }
    }
}

/// Copy the .opencode directory, including only specific files.
fn copy_opencode_dir(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;

    // Copy opencode.jsonc
    let src_config = src.join("opencode.jsonc");
    if src_config.exists() {
        std::fs::copy(&src_config, dst.join("opencode.jsonc"))?;
    }

    // Copy commands directory
    let src_commands = src.join("commands");
    if src_commands.exists() {
        copy_dir_recursive(&src_commands, &dst.join("commands"))?;
    }

    // Copy plugin directory
    let src_plugin = src.join("plugin");
    if src_plugin.exists() {
        copy_dir_recursive(&src_plugin, &dst.join("plugin"))?;
    }

    Ok(())
}

/// Recursively copy a directory.
fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_dir_returns_some() {
        assert!(data_dir().is_some());
    }

    #[test]
    fn test_data_dir_contains_persona() {
        let dir = data_dir().unwrap();
        assert!(dir.ends_with("persona"));
    }

    #[test]
    fn test_is_dev_mode_with_env_var() {
        std::env::set_var("PERSONA_DEV", "1");
        assert!(is_dev_mode());
        std::env::remove_var("PERSONA_DEV");
    }

    #[test]
    fn test_working_dir_in_dev_mode() {
        std::env::set_var("PERSONA_DEV", "1");
        let wd = working_dir();
        // In dev mode, should return current directory
        assert!(wd.exists());
        std::env::remove_var("PERSONA_DEV");
    }
}
