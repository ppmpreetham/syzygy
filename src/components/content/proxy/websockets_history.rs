use gpui_kit::*;
use zopra::{component, view};

#[component]
pub fn websockets_history() {
  view! {
      <div class="flex flex-col size-full items-center justify-center text-[#ededed]">
          "WebSockets History Component"
      </div>
  }
}
