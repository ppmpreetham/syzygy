
use gpui_kit::*;
use zopra::{view, component};

#[component]
pub fn http_history() {
    view! {
        <div class="flex flex-col size-full items-center justify-center text-[#ededed]">
            "HTTP History Component"
        </div>
    }
}

