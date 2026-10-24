use gpui_kit::*;
use std::sync::Arc;
use strum_macros::FromRepr;
use zopra::ui::{Tab, TabBar};
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
        <div class="flex flex-col size-full">
            <TabBar
                underline
                selected_index={active_tab() as usize}
                on_click={|index, _, cx| {
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
            </TabBar>

            <div class="flex-1 w-full text-[#ededed]">
                {match active_tab() {
                    ProxyTab::Intercept => view! { <Intercept proxy_state={proxy_state} /> },
                    ProxyTab::HttpHistory => view! { <HttpHistory proxy_state={proxy_state} /> },
                    ProxyTab::WebSocketsHistory => view! { <WebsocketsHistory /> },
                    ProxyTab::Options => view! { <Options /> },
                }}
            </div>
        </div>
    }
}
