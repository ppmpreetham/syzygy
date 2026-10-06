use gpui_kit::*;
use zopra::{component, view};

#[component]
pub fn intruder() {
    view! {
        <div class="flex flex-col items-center justify-center size-full text-[#ededed] text-xl">
            <div></div>
            <div class="flex flex-row "></div>
        </div>
    }
}
