use gpui_kit::*;
use std::sync::Arc;
use strum_macros::FromRepr;
use zopra::{component, hooks::use_state, view};

pub mod http_history;
pub mod intercept;
pub mod options;
pub mod websockets_history;

use crate::backend::ProxyState;

use self::http_history::HttpHistory;
use self::intercept::Intercept;
use self::options::Options;
use self::websockets_history::WebsocketsHistory;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub enum ProxyTab {
    Intercept = 0,
    HttpHistory = 1,
    WebSocketsHistory = 2,
    Options = 3,
}

#[component]
pub fn proxy(proxy_state: Arc<ProxyState>) {
    let (active_tab, set_active_tab) = use_state(ProxyTab::Intercept);

    view! {
        <div class="flex flex-col size-full bg-[#141517]">
            <nav
                underline
                selected_index={*active_tab as usize}
                on_click={move |index, _, cx| {
                    if let Some(tab) = ProxyTab::from_repr(*index) {
                        set_active_tab(tab);
                    }
                }}
                class="px-4"
            >
                <Tab label="Intercept" />
                <Tab label="HTTP history" />
                <Tab label="WebSockets history" />
                <Tab label="Options" />
            </nav>

            <div class="flex-1 w-full text-[#ededed]">
                {match *active_tab {
                    ProxyTab::Intercept => view! { <Intercept proxy_state={proxy_state.clone()} /> }.into_any_element(),
                    ProxyTab::HttpHistory => view! { <HttpHistory proxy_state={proxy_state.clone()} /> }.into_any_element(),
                    ProxyTab::WebSocketsHistory => view! { <WebsocketsHistory /> }.into_any_element(),
                    ProxyTab::Options => view! { <Options /> }.into_any_element(),
                }}
            </div>
        </div>
    }
}
