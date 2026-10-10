use crate::components::content::MainContent;
use crate::components::sidebar::AppSidebar;
use crate::components::titlebar::AppTitleBar;
use crate::{backend::ProxyState, components::AppTab};

use gpui_kit::*;

use std::sync::Arc;
use zopra::{component, hooks::use_state, view};
use crate::components::content::intruder::state::IntruderState;

#[component]
pub fn app(proxy_state: &Arc<ProxyState>) {
    let (collapsed, set_collapsed) = use_state(false);
    let (app_tab, set_app_tab) = use_state(AppTab::Proxy);
    let (intruders, set_intruders) = use_state(vec![IntruderState::default()]);
    let (intruder_tab, set_intruder_tab) = use_state(0usize);

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
                            set_active_tab={set_app_tab.clone()}
                        />
                    </ResizablePanel>
                    <ResizablePanel>
                        <MainContent active_tab={*app_tab} set_app_tab={set_app_tab.clone()} proxy_state={proxy_state.clone()} intruders={intruders.clone()} set_intruders={set_intruders.clone()} intruder_tab={intruder_tab.clone()} set_intruder_tab={set_intruder_tab.clone()} />
                    </ResizablePanel>
                </Resizable>
            </div>
        </div>
    }
}
