#![deny(clippy::absolute_paths)]

mod app;
mod assets;
mod backend;
pub mod components;
pub mod globals;

use crate::backend::{ProxyState, start_proxy};
use crate::components::content::settings::config;
use app::app;
use assets::AppAssets;
use components::content::settings::theme::theme_init;
use components::content::settings::config_init;
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
        config_init(cx).ok();
        theme_init(cx).ok();

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Maximized(Bounds { origin: point(px(0.0), px(0.0)), size: size(px(1920.0), px(1080.0)) })),
            window_background: cx.global::<config::Config>().window_style.into(),
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

