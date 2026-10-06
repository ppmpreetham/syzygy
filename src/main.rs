#![deny(clippy::absolute_paths)]

mod app;
mod assets;
mod backend;
pub mod components;
mod config;

use crate::backend::{ProxyState, start_proxy};
use app::app;
use assets::AppAssets;
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::component::{Root, TitleBar};
use gpui_kit::gpui::WindowBackgroundAppearance;
use gpui_kit::*;
use std::sync::Arc;
use std::{future, thread};
use tokio::runtime;

pub struct Main {
    proxy_state: Arc<ProxyState>,
}
impl Render for Main {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        app(&self.proxy_state, window, cx)
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(AppAssets);
    let config = config::Config::new();
    let proxy_state = ProxyState::new();

    app.run(move |cx| {
        let proxy_for_server = Arc::clone(&proxy_state);
        thread::spawn(move || {
            let rt = runtime::Runtime::new().unwrap();
            rt.block_on(async {
                start_proxy(proxy_for_server).await;
                future::pending::<()>().await;
            });
        });

        gpui_kit::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);

        let window_options = WindowOptions {
            window_bounds: Some(config.window_size),
            window_background: WindowBackgroundAppearance::Opaque,
            ..TitleBar::window_options()
        };

        cx.open_window(window_options, |window, cx| {
            let state = Arc::clone(&proxy_state);
            let view = cx.new(|_| Main { proxy_state: state });
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("failed to open window");
        cx.activate(true);
    });
}
