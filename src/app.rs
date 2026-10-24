use crate::components::content::MainContent;
use crate::components::sidebar::AppSidebar;
use crate::components::titlebar::AppTitleBar;
use crate::{backend::ProxyState, components::AppTab};

use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

use crate::components::content::intruder::state::Intruders;
use std::sync::Arc;
use zopra::ui::{Resizable, ResizablePanel};
use zopra::{
    component,
    hooks::{provide_context, use_signal, use_state},
    view,
};

#[component]
pub fn app(proxy_state: &Arc<ProxyState>) {
    let (collapsed, set_collapsed) = use_state(false);
    let app_tab = use_signal(AppTab::Proxy);
    let intruders = use_signal(Intruders::default());
    provide_context(app_tab);
    provide_context(intruders);

    view! {
        <div class="flex flex-col size-full bg-theme-background text-theme-foreground">
            <AppTitleBar />
            <div class="flex-1 w-full relative">
                <Resizable id="main-layout" horizontal
                    on_resize={|state, _window, cx| {
                        if let Some(sidebar_size) = state.read(cx).sizes().first() {
                            let is_small = *sidebar_size <= px(56.);
                            if is_small != collapsed() {
                                set_collapsed(is_small);
                            }
                        }
                    }}
                >
                    <ResizablePanel
                        size={px(250.)}
                        size_range={px(55.)..px(600.)}
                    >
                        <AppSidebar collapsed={collapsed()} active_tab={app_tab} />
                    </ResizablePanel>
                    <ResizablePanel>
                        <MainContent active_tab={app_tab()} proxy_state={proxy_state.clone()} />
                    </ResizablePanel>
                </Resizable>
            </div>
        </div>
    }
}
