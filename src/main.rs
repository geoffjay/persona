mod app;
mod config;
mod http;
mod knowledgebase;
mod memory;
mod persona;
mod state;
mod ui;

use app::App;
use config::{ensure_data_dir, AppConfig};
use gpui::*;
use gpui_component::Root;

fn main() {
    let app = Application::new()
        .with_assets(gpui_component_assets::Assets)
        .with_http_client(http::ReqwestHttpClient::new());
    app.run(move |cx| {
        gpui_tokio_bridge::init(cx);
        gpui_component::init(cx);

        // Ensure data directory exists and bootstrap from bundled resources if needed
        // This copies personas and .opencode config on first run
        if let Some(data_dir) = ensure_data_dir() {
            eprintln!("Using data directory: {:?}", data_dir);
        }

        // Load configuration from TOML file with environment variable overrides
        let config = AppConfig::load();

        let window_size = size(px(1200.), px(800.));
        let bounds = Bounds::centered(None, window_size, cx);

        // Quit the application when the window is closed
        cx.on_window_closed(|cx| {
            cx.quit();
        })
        .detach();

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
