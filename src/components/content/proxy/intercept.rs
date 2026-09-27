
use gpui_kit::*;
use zopra::{view, component};

#[component]
pub fn intercept() {
    view! {
        <div class="flex flex-col size-full items-center justify-center text-[#ededed]">
            "Intercept Component"
        </div>
    }
}

