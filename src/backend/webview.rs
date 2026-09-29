use wry::{ProxyConfig, ProxyEndpoint, WebViewBuilder};

// pub fn webview_builder<W>(url: &str, proxy: ProxyConfig, window: &W) -> wry::WebView
// where
//     W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
// {
//     WebViewBuilder::new()
//         .with_url(url)
//         .with_proxy_config(proxy)
//         .build(window)
//         .unwrap()
// }

pub fn proxy_config() -> ProxyConfig {
  ProxyConfig::Http(ProxyEndpoint {
    host: "127.0.0.1".into(),
    port: "8080".into(),
  })
}
