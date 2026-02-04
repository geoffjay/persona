use gpui::*;
use gpui_component::TitleBar;

pub fn get_window_options(cx: &mut App) -> WindowOptions {
    // Default window size when restored from maximized state
    let restored_size = size(px(1200.), px(800.));
    let restored_bounds = Bounds::centered(None, restored_size, cx);

    WindowOptions {
        window_bounds: Some(WindowBounds::Maximized(restored_bounds)),
        titlebar: Some(TitleBar::title_bar_options()),
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
    }
}
