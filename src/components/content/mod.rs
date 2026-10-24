pub mod comparer;
pub mod dashboard;
pub mod decoder;
pub mod extensions;
pub mod intruder;
pub mod logger;
pub mod organizer;
pub mod proxy;
pub mod repeater;
pub mod settings;
pub mod target;

use crate::backend::ProxyState;
use crate::components::AppTab;
use gpui_kit::*;
use std::sync::Arc;
use zopra::{component, view};

use self::comparer::Comparer;
use self::dashboard::Dashboard;
use self::decoder::Decoder;
use self::extensions::Extensions;
use self::logger::Logger;
use self::organizer::Organizer;
use self::proxy::Proxy;
use self::repeater::Repeater;
use self::target::Target;
use intruder::Intruder;

#[component]
pub fn main_content(active_tab: AppTab, proxy_state: Arc<ProxyState>) {
    view! {
        <div class="flex-1 size-full flex flex-col">
            {match active_tab {
                AppTab::Dashboard => view! { <Dashboard /> },
                AppTab::Target => view! { <Target /> },
                AppTab::Proxy => view! { <Proxy proxy_state={proxy_state} /> },
                AppTab::Intruder => view! { <Intruder /> },
                AppTab::Repeater => view! { <Repeater /> },
                AppTab::Decoder => view! { <Decoder /> },
                AppTab::Comparer => view! { <Comparer /> },
                AppTab::Logger => view! { <Logger /> },
                AppTab::Organizer => view! { <Organizer /> },
                AppTab::Extensions => view! { <Extensions /> },
            }}
        </div>
    }
}
