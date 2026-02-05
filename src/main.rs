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
use std::cell::RefCell;
use std::rc::Rc;
use ui::{theme::apply_theme, window::get_window_options};

fn main() {
    env_logger::init();

    let app = Application::new()
        .with_assets(gpui_component_assets::Assets)
        .with_http_client(http::ReqwestHttpClient::new());

    app.run(move |cx| {
        gpui_tokio_bridge::init(cx);
        gpui_component::init(cx);

        // Unbind Tab from focus navigation so it passes through to terminals
        cx.bind_keys([
            KeyBinding::new("tab", NoAction, Some("Root")),
            KeyBinding::new("shift-tab", NoAction, Some("Root")),
        ]);

        // Ensure data directory exists and bootstrap from bundled resources if needed
        // This copies personas and .opencode config on first run
        if let Some(data_dir) = ensure_data_dir() {
            eprintln!("Using data directory: {:?}", data_dir);
        }

        let config = AppConfig::load();

        // Store the App entity so we can access it on window close
        let app_entity: Rc<RefCell<Option<Entity<App>>>> = Rc::new(RefCell::new(None));

        // Close all sessions and quit the application when the window is closed
        cx.on_window_closed({
            let app_entity = app_entity.clone();
            move |cx| {
                // Shutdown the app to close any open sessions
                if let Some(app) = app_entity.borrow().clone() {
                    app.update(cx, |app, app_cx| {
                        app.shutdown(app_cx);
                    });
                }
                cx.quit();
            }
        })
        .detach();

        let window_options = get_window_options(cx);
        cx.open_window(window_options, |window, cx| {
            apply_theme(&config.general.theme, cx);

            let view = cx.new(|cx| App::new(config, window, cx));
            *app_entity.borrow_mut() = Some(view.clone());
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
    });
}
