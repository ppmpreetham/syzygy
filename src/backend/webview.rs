use zopra::lb_wry::{ProxyConfig, ProxyEndpoint};

pub fn proxy_config() -> ProxyConfig {
  ProxyConfig::Http(ProxyEndpoint {
    host: "127.0.0.1".into(),
    port: "8080".into(),
  })
}
