use gpui_kit::assets::IconName;
use gpui_kit::component::Icon;
use gpui_kit::component::Sizable;
use gpui_kit::component::TitleBar;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use zopra::{component, view};

#[component]
pub fn app_title_bar() {
  view! {
      <TitleBar class="border-b border-[#272a2f] bg-[#141517]">
          <div class="flex items-center gap-2 px-2 text-[#d9dbe0]">
              <icon name={IconName::PanelLeft} small text_color={rgb(0x777b83)} />
              <div class="text-xs font-semibold">"Syzygy"</div>
          </div>
          // spacer
          <div class="flex items-center flex-1 justify-end px-2 gap-2 text-[#d9dbe0]">
          </div>
      </TitleBar>
  }
}
