use gpui_kit::*;

pub struct Config {
  pub window_size: WindowBounds,
}

impl Config {
  pub fn new() -> Self {
    Self {
      window_size: WindowBounds::Maximized(Bounds {
        origin: point(px(0.0), px(0.0)),
        size: size(px(1920.0), px(1080.0)),
      }),
    }
  }
}
