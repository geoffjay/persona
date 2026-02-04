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
use gpui_component::{ActiveTheme as _, Root};
use ui::{theme::change_color_mode, window::get_window_options};

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

        // Quit the application when the window is closed
        cx.on_window_closed(|cx| {
            cx.quit();
        })
        .detach();

        let window_options = get_window_options(cx);
        cx.open_window(window_options, |window, cx| {
            change_color_mode(cx.theme().mode, window, cx);

            let view = cx.new(|cx| App::new(config, window, cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
    });
}
