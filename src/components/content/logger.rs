use gpui_kit::*;
use zopra::{component, view};

#[component]
pub fn logger() {
  view! {
      <div class="flex items-center justify-center size-full text-[#ededed] text-xl">
          "Logger Stub"
      </div>
  }
}
