use gpui_kit::*;
use zopra::{component, view};

#[component]
pub fn app_title_bar() {
    view! {
        <TitleBar class="border-b border-[#272a2f] bg-[#141517]">
            <div class="flex items-center gap-2 px-2 text-[#d9dbe0]">
                <img src="/syzygy.png" class="size-4" />
                <div class="text-xs font-semibold">"Syzygy"</div>
            </div>
            // spacer
            <div class="flex items-center flex-1 justify-end px-2 gap-2 text-[#d9dbe0]">
            </div>
        </TitleBar>
    }
}
