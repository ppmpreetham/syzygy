use crate::components::AppTab;
use crate::components::content::MainContent;
use crate::components::sidebar::AppSidebar;
use crate::components::titlebar::AppTitleBar;
use gpui_kit::component::resizable::*;
use gpui_kit::*;
use gpui_kit::{component::TitleBar, prelude::FluentBuilder};
use std::rc::Rc;
use zopra::{cn, component, hooks::use_state, view};

#[component]
pub fn app() {
  let (get_collapsed, set_collapsed) = use_state(false);
  let collapsed_val = get_collapsed();

  let (get_app_tab, set_app_tab) = use_state(AppTab::Proxy);
  let app_tab_val = get_app_tab();
  let set_app_tab_rc = Rc::new(move |val, cx: &mut gpui_kit::App| set_app_tab(val, cx));

  view! {
      <div class="flex flex-col size-full bg-[#141517] text-[#ededed]">
          <AppTitleBar />

          <div class="flex-1 w-full relative">
              <Resizable id="main-layout" horizontal
                  on_resize={move |state, _window, cx| {
                      if let Some(sidebar_size) = state.read(cx).sizes().first() {
                          let is_small = *sidebar_size <= px(56.);
                          if is_small != collapsed_val {
                              set_collapsed(is_small, cx);
                          }
                      }
                  }}
              >
                  <ResizablePanel
                      size={px(250.)}
                      size_range={px(55.)..px(600.)}
                  >
                      <AppSidebar
                          collapsed={collapsed_val}
                          active_tab={app_tab_val}
                          set_active_tab={set_app_tab_rc}
                      />
                  </ResizablePanel>
                  <ResizablePanel>
                      <MainContent active_tab={app_tab_val} />
                  </ResizablePanel>
              </Resizable>
          </div>
      </div>
  }
}
