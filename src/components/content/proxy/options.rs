
use gpui_kit::*;
use zopra::{view, component};

#[component]
pub fn options() {
    view! {
        <div class="flex flex-col size-full items-center justify-center text-[#ededed]">
            "Options Component"
        </div>
    }
}

