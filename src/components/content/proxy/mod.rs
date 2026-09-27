
use gpui_kit::*;
use zopra::{view, component, hooks::use_state};
use strum_macros::FromRepr;

pub mod intercept;
pub mod http_history;
pub mod websockets_history;
pub mod options;

use self::intercept::{Intercept, InterceptProps};
use self::http_history::{HttpHistory, HttpHistoryProps};
use self::websockets_history::{WebsocketsHistory, WebsocketsHistoryProps};
use self::options::{Options, OptionsProps};

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub enum ProxyTab {
    Intercept = 0,
    HttpHistory = 1,
    WebSocketsHistory = 2,
    Options = 3,
}

#[component]
pub fn proxy() {
    let (active_tab, set_active_tab) = use_state(ProxyTab::Intercept);
    let tab_val = active_tab();

    view! {
        <div class="flex flex-col size-full bg-[#141517]">
            <nav
                underline
                selected_index={tab_val as usize}
                on_click={move |index, _, cx| {
                    if let Some(tab) = ProxyTab::from_repr(*index) {
                        set_active_tab(tab, cx);
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
                {match tab_val {
                    ProxyTab::Intercept => view! { <Intercept /> }.into_any_element(),
                    ProxyTab::HttpHistory => view! { <HttpHistory /> }.into_any_element(),
                    ProxyTab::WebSocketsHistory => view! { <WebsocketsHistory /> }.into_any_element(),
                    ProxyTab::Options => view! { <Options /> }.into_any_element(),
                }}
            </div>
        </div>
    }
}
