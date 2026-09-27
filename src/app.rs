use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use gpui_kit::component::resizable::*;
use zopra::{component, hooks::use_state, view, cn};
use crate::components::sidebar::{app_sidebar, AppSidebarProps};
use crate::components::content::{main_content, MainContentProps};
use crate::components::titlebar::{AppTitleBar, AppTitleBarProps};

#[component]
pub fn app() {
    let (get_collapsed, set_collapsed) = use_state(false);
    let collapsed_val = get_collapsed();

    view! {
        <div class="flex flex-col size-full bg-[#141517] text-[#ededed]">
            <AppTitleBar />

            <div class="flex-1 w-full relative">
                <Resizable id="main-layout" horizontal>
                    <ResizablePanel
                        size={px(250.)}
                        size_range={px(55.)..px(600.)}
                    >
                        <AppSidebar collapsed={collapsed_val} />
                    </ResizablePanel>
                    <ResizablePanel>
                        <MainContent />
                    </ResizablePanel>
                </Resizable>
            </div>
        </div>
    }
}
