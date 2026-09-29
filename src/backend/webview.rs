use wry::{ProxyConfig, ProxyEndpoint, WebViewBuilder};

pub fn webview_builder(url: &str, proxy: ProxyConfig) -> wry::WebView {
    WebViewBuilder::new()
        .with_url(url)
        .with_proxy_config(proxy)
        .build(&window)
        .unwrap()
}

pub fn proxy_config() -> ProxyConfig {
    ProxyConfig::Http(ProxyEndpoint {
        host: "127.0.0.1".into(),
        port: "8080".into(),
    })
}
