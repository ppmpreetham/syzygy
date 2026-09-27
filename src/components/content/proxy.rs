use gpui_kit::*;
use zopra::{view, component, hooks::use_state};
use strum_macros::FromRepr;

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
                pill
                selected_index={tab_val as usize}
                on_click={move |index, _, cx| {
                    if let Some(tab) = ProxyTab::from_repr(*index) {
                        set_active_tab(tab, cx);
                    }
                }}
            >
                <Tab label="Intercept" />
                <Tab label="HTTP history" />
                <Tab label="WebSockets history" />
                <Tab label="Options" />
            </nav>

            <div class="flex-1 w-full p-4 text-[#ededed]">
                {match tab_val {
                    ProxyTab::Intercept => view! {
                        <div class="flex items-center justify-center size-full">
                            "Intercept is on"
                        </div>
                    }.into_any_element(),
                    ProxyTab::HttpHistory => view! {
                        <div class="flex items-center justify-center size-full">
                            "HTTP History Table Here"
                        </div>
                    }.into_any_element(),
                    ProxyTab::WebSocketsHistory => view! {
                        <div class="flex items-center justify-center size-full">
                            "WebSockets History Table Here"
                        </div>
                    }.into_any_element(),
                    ProxyTab::Options => view! {
                        <div class="flex items-center justify-center size-full">
                            "Proxy Options & Settings"
                        </div>
                    }.into_any_element(),
                }}
            </div>
        </div>
    }
}
