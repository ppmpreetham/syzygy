use crate::backend::betterproxy::ProxyState;
use crate::components::AppTab;
use crate::components::content::MainContent;
use crate::components::sidebar::AppSidebar;
use crate::components::titlebar::AppTitleBar;
use gpui_kit::*;
use std::sync::Arc;
use zopra::{component, hooks::use_state, view};

#[component]
pub fn app(proxy_state: Arc<ProxyState>) {
    let (collapsed, set_collapsed) = use_state(false);
    let (app_tab, set_app_tab) = use_state(AppTab::Proxy);

    view! {
        <div class="flex flex-col size-full bg-[#141517] text-[#ededed]">
            <AppTitleBar />
            <div class="flex-1 w-full relative">
                <Resizable id="main-layout" horizontal
                    on_resize={move |state, _window, cx| {
                        if let Some(sidebar_size) = state.read(cx).sizes().first() {
                            let is_small = *sidebar_size <= px(56.);
                            if is_small != collapsed {
                                set_collapsed(is_small);
                            }
                        }
                    }}
                >
                    <ResizablePanel
                        size={px(250.)}
                        size_range={px(55.)..px(600.)}
                    >
                        <AppSidebar
                            collapsed={*collapsed}
                            active_tab={*app_tab}
                            set_active_tab={set_app_tab}
                        />
                    </ResizablePanel>
                    <ResizablePanel>
                        <MainContent active_tab={*app_tab} proxy_state={proxy_state.clone()} />
                    </ResizablePanel>
                </Resizable>
            </div>
        </div>
    }
}
