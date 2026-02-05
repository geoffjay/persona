mod footer_bar;
mod header_bar;

pub use footer_bar::FooterBar;
pub use header_bar::HeaderBar;

use gpui::*;

// pub struct Workspace {
//     header_bar: HeaderBar,
//     footer_bar: FooterBar,
//     _subscriptions: Vec<Subscription>,
// }
//
// impl Workspace {
//     pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//         let header_bar = HeaderBar::new(window, cx);
//         let footer_bar = FooterBar::new(window, cx);
//         let _subscriptions = vec![];
//
//         Self {
//             header_bar,
//             footer_bar,
//             _subscriptions,
//         }
//     }
//
//     pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
//         cx.new(|cx| Self::new(window, cx))
//     }
// }

// impl Render for Workspace {
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {}
// }
