pub mod comparer;
pub mod dashboard;
pub mod decoder;
pub mod extensions;
pub mod intruder;
pub mod logger;
pub mod organizer;
pub mod proxy;
pub mod repeater;
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
use self::intruder::Intruder;
use self::logger::Logger;
use self::organizer::Organizer;
use self::proxy::Proxy;
use self::repeater::Repeater;
use self::target::Target;

#[component]
pub fn main_content(active_tab: AppTab, proxy_state: Arc<ProxyState>) {
    view! {
        <div class="flex-1 size-full flex flex-col">
            {match active_tab {
                AppTab::Dashboard => view! { <Dashboard /> }.into_any_element(),
                AppTab::Target => view! { <Target /> }.into_any_element(),
                AppTab::Proxy => view! { <Proxy proxy_state={proxy_state.clone()} /> }.into_any_element(),
                AppTab::Intruder => view! { <Intruder /> }.into_any_element(),
                AppTab::Repeater => view! { <Repeater /> }.into_any_element(),
                AppTab::Decoder => view! { <Decoder /> }.into_any_element(),
                AppTab::Comparer => view! { <Comparer /> }.into_any_element(),
                AppTab::Logger => view! { <Logger /> }.into_any_element(),
                AppTab::Organizer => view! { <Organizer /> }.into_any_element(),
                AppTab::Extensions => view! { <Extensions /> }.into_any_element(),
            }}
        </div>
    }
}
