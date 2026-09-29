use crate::components::AppTab;
use gpui_kit::component::Icon;
use gpui_kit::component::Sizable;
use gpui_kit::component::button::*;
use gpui_kit::component::sidebar::*;
use gpui_kit::*;
use gpui_kit::{assets::IconName, prelude::FluentBuilder};
use std::rc::Rc;
use zopra::{component, view};

#[component]
pub fn user_section() {
  view! {
      <div class="mt-auto h-[40px] flex items-center px-[6px] rounded-[7px] hover:bg-[#1d2024] w-full">
          <div class="size-[26px] rounded-full bg-blue-500" />
          <div class="ml-2 flex-1 flex flex-col justify-center gap-[1px]">
              <div class="text-[#d0d2d6] text-[11px]">"Preetham"</div>
              <div class="text-[#656970] text-[9px]">"Thing"</div>
          </div>
          <icon name={IconName::EllipsisVertical} small text_color={rgb(0x656970)} />
      </div>
  }
}

#[component]
pub fn app_sidebar(
  collapsed: bool,
  active_tab: AppTab,
  set_active_tab: Rc<dyn Fn(AppTab, &mut App)>,
) {
  view! {
      <Sidebar collapsed={collapsed} collapsible={SidebarCollapsible::Icon}
          class="w-full h-full bg-[#141517] border-r border-[#272a2f]"
          footer={view! {
              <SidebarFooter>
                  <UserSection />
              </SidebarFooter>
          }}
      >
          <SidebarGroup label="">
              <SidebarMenu>
                  <SidebarMenuItem icon={Icon::default().path("icons/dashboard.svg")} label="Dashboard" active={active_tab == AppTab::Dashboard} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Dashboard, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/target.svg")} label="Target" active={active_tab == AppTab::Target} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Target, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/proxy.svg")} label="Proxy" active={active_tab == AppTab::Proxy} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Proxy, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/intruder.svg")} label="Intruder" active={active_tab == AppTab::Intruder} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Intruder, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/repeater.svg")} label="Repeater" active={active_tab == AppTab::Repeater} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Repeater, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/decoder.svg")} label="Decoder" active={active_tab == AppTab::Decoder} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Decoder, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/comparer.svg")} label="Comparer" active={active_tab == AppTab::Comparer} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Comparer, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/logger.svg")} label="Logger" active={active_tab == AppTab::Logger} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Logger, cx)
                  }} />
                  <SidebarMenuItem icon={IconName::Folder} label="Organizer" active={active_tab == AppTab::Organizer} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Organizer, cx)
                  }} />
                  <SidebarMenuItem icon={Icon::default().path("icons/extension.svg")} label="Extensions" active={active_tab == AppTab::Extensions} on_click={{
                      let setter = set_active_tab.clone();
                      move |_, _, cx| setter(AppTab::Extensions, cx)
                  }} />
              </SidebarMenu>
          </SidebarGroup>
      </Sidebar>
  }
}
