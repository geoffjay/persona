mod app;
mod config;
mod persona;
mod state;
mod terminal;
mod ui;

use app::App;
use config::AppConfig;
use gpui::*;
use gpui_component::Root;
use std::path::PathBuf;

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);
    app.run(move |cx| {
        gpui_component::init(cx);

        // Configure personas directory
        let personas_dir = std::env::var("PERSONAS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::current_dir()
                    .unwrap_or_default()
                    .join("personas")
            });

        let config = AppConfig::default().with_personas_dir(personas_dir);

        let window_size = size(px(1200.), px(800.));
        let bounds = Bounds::centered(None, window_size, cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Persona UI".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| App::new(config, window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .expect("Failed to open window");
    });
}
