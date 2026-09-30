mod app;
mod assets;
mod backend;
pub mod components;
mod config;

use app::app;
use assets::AppAssets;
use gpui_kit::component::{Root, TitleBar};
use gpui_kit::gpui::WindowBackgroundAppearance;
use gpui_kit::*;

pub struct Main;
impl Render for Main {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    app(window, cx)
  }
}

fn main() {
  let app = gpui_kit::application().with_assets(AppAssets);
  let config = config::Config::new();

  std::thread::spawn(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      crate::backend::proxy::start_proxy().await;
      std::future::pending::<()>().await;
    });
  });
  app.run(move |cx| {
    gpui_kit::init(cx);
    gpui_kit::component::theme::Theme::change(
      gpui_kit::component::theme::ThemeMode::Dark,
      None,
      cx,
    );

    let window_options = WindowOptions {
      window_bounds: Some(config.window_size),
      window_background: WindowBackgroundAppearance::Opaque,
      ..TitleBar::window_options()
    };

    cx.open_window(window_options, |window, cx| {
      let view = cx.new(|_| Main);
      cx.new(|cx| Root::new(view, window, cx))
    })
    .expect("failed to open window");
    cx.activate(true);
  });
}
