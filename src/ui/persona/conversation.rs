use crate::config::{working_dir, AppConfig};
use crate::persona::Persona;
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
    needs_focus: bool,
}

impl ConversationView {
    pub fn new(persona: Persona, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut view = Self {
            persona: persona.clone(),
            terminal: None,
            pty_master: None,
            error: None,
            needs_focus: true,
        };

        if let Err(e) = view.spawn_terminal(&persona, window, cx) {
            view.error = Some(format!("Failed to spawn terminal: {}", e));
        }

        view
    }

    /// Request focus for the terminal
    pub fn focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(terminal) = &self.terminal {
            let focus_handle = terminal.read(cx).focus_handle().clone();
            window.focus(&focus_handle);
        }
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
        // Pass the persona ID which matches the agent name in opencode's config
        let mut cmd = CommandBuilder::new("opencode");
        cmd.arg("--agent");
        cmd.arg(&persona.id);
        cmd.arg("--continue");

        // Set the working directory to where .opencode/opencode.jsonc lives
        // In dev mode: project root; in production: ~/Library/Application Support/persona
        cmd.cwd(working_dir());

        // Spawn the command in the PTY
        pair.slave.spawn_command(cmd)?;

        // Load terminal configuration from unified app config
        let app_config = AppConfig::load();
        let config = app_config
            .terminal
            .to_terminal_config(initial_cols as usize, initial_rows as usize);

        let terminal = cx.new({
            let pty = master.clone();
            move |inner_cx| {
                TerminalView::new(writer, reader, config, inner_cx).with_resize_callback(
                    move |cols, rows| {
                        if let Ok(guard) = pty.lock() {
                            let _ = guard.resize(PtySize {
                                rows: rows as u16,
                                cols: cols as u16,
                                pixel_width: 0,
                                pixel_height: 0,
                            });
                        }
                    },
                )
            }
        });

        self.terminal = Some(terminal);
        Ok(())
    }
}

impl Render for ConversationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(error) = &self.error {
            div()
                .id("conversation-view")
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(cx.theme().background)
                .text_color(gpui::red())
                .child(error.clone())
                .into_any_element()
        } else if let Some(terminal) = &self.terminal {
            // Focus the terminal on first render so it receives keyboard input
            if self.needs_focus {
                let focus_handle = terminal.read(cx).focus_handle().clone();
                window.focus(&focus_handle);
                self.needs_focus = false;
            }

            // Wrap terminal in a flex container to ensure it fills available space
            div()
                .flex_1()
                .size_full()
                .child(terminal.clone())
                .into_any_element()
        } else {
            div()
                .id("conversation-view")
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(cx.theme().background)
                .child("Loading terminal...")
                .into_any_element()
        }
    }
}
