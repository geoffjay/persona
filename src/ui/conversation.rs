use crate::persona::Persona;
use crate::terminal::AppTerminalConfig;
use anyhow::Result;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_terminal::TerminalView;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::sync::{Arc, Mutex};

pub struct ConversationView {
    #[allow(dead_code)]
    persona: Persona,
    terminal: Option<Entity<TerminalView>>,
    #[allow(dead_code)]
    pty_master: Option<Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>>,
    error: Option<String>,
}

impl ConversationView {
    pub fn new(persona: Persona, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut view = Self {
            persona: persona.clone(),
            terminal: None,
            pty_master: None,
            error: None,
        };

        if let Err(e) = view.spawn_terminal(&persona, window, cx) {
            view.error = Some(format!("Failed to spawn terminal: {}", e));
        }

        view
    }

    fn spawn_terminal(
        &mut self,
        persona: &Persona,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let pty_system = native_pty_system();

        let initial_rows = 24;
        let initial_cols = 80;

        let pair = pty_system.openpty(PtySize {
            rows: initial_rows,
            cols: initial_cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let writer = pair.master.take_writer()?;
        let reader = pair.master.try_clone_reader()?;

        let master = Arc::new(Mutex::new(pair.master));
        self.pty_master = Some(master.clone());

        // Build the command to start opencode with the persona
        let mut cmd = CommandBuilder::new("opencode");
        cmd.arg("--agent");
        cmd.arg(persona.file_path.to_string_lossy().as_ref());

        // Spawn the command in the PTY
        pair.slave.spawn_command(cmd)?;

        // Load terminal configuration (with Tokyo Night dark as default)
        let terminal_config = AppTerminalConfig::load();
        let config = terminal_config.to_terminal_config(initial_cols as usize, initial_rows as usize);

        let terminal = cx.new({
            let pty = master.clone();
            move |inner_cx| {
                TerminalView::new(writer, reader, config, inner_cx)
                    .with_resize_callback(move |cols, rows| {
                        if let Ok(guard) = pty.lock() {
                            let _ = guard.resize(PtySize {
                                rows: rows as u16,
                                cols: cols as u16,
                                pixel_width: 0,
                                pixel_height: 0,
                            });
                        }
                    })
            }
        });

        self.terminal = Some(terminal);
        Ok(())
    }
}

impl Render for ConversationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = if let Some(error) = &self.error {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(gpui::red())
                .child(error.clone())
                .into_any_element()
        } else if let Some(terminal) = &self.terminal {
            terminal.clone().into_any_element()
        } else {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child("Loading terminal...")
                .into_any_element()
        };

        div()
            .id("conversation-view")
            .size_full()
            .bg(cx.theme().background)
            .child(content)
    }
}
