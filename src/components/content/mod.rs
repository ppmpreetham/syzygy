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

use crate::components::AppTab;
use gpui_kit::*;
use zopra::{component, view};

use self::comparer::{Comparer, ComparerProps};
use self::dashboard::{Dashboard, DashboardProps};
use self::decoder::{Decoder, DecoderProps};
use self::extensions::{Extensions, ExtensionsProps};
use self::intruder::{Intruder, IntruderProps};
use self::logger::{Logger, LoggerProps};
use self::organizer::{Organizer, OrganizerProps};
use self::proxy::{Proxy, ProxyProps};
use self::repeater::{Repeater, RepeaterProps};
use self::target::{Target, TargetProps};

#[component]
pub fn main_content(active_tab: AppTab) {
  view! {
      <div class="flex-1 size-full flex flex-col">
          {match active_tab {
              AppTab::Dashboard => view! { <Dashboard /> }.into_any_element(),
              AppTab::Target => view! { <Target /> }.into_any_element(),
              AppTab::Proxy => view! { <Proxy /> }.into_any_element(),
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
