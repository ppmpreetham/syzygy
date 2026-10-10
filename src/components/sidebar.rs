use crate::components::AppTab;
use gpui_kit::assets::IconName;
use gpui_kit::component::Icon;
use gpui_kit::component::Sizable;
use gpui_kit::component::sidebar::*;
use gpui_kit::*;
use zopra::hooks::use_window;
use gpui_kit::component::TitleBar;
use zopra::{component, hooks::Setter, view};
use gpui_kit::component::dialog::{DialogHeader, DialogTitle};
use crate::components::content::settings::menu::SettingsMenu;

#[component]
pub fn user_section() {
    let settings_window = use_window(|| gpui_kit::gpui::WindowOptions {
        window_bounds: Some(gpui_kit::gpui::WindowBounds::Windowed(gpui_kit::gpui::Bounds {
            origin: Default::default(),
            size: gpui_kit::gpui::size(gpui_kit::gpui::px(800.0), gpui_kit::gpui::px(600.0)),
        })),
        ..TitleBar::window_options()
    });
    view! {
        <div class="mt-auto h-[40px] flex items-center px-[6px] rounded-[7px] hover:bg-[#1d2024] w-full cursor-pointer" on_click={{ move |_, _, cx| {
                settings_window.open(|window, cx| {
                    view! {
                        <div class="w-full h-full flex flex-col bg-[#1e1e24]">
                            <TitleBar class="border-b border-[#272a2f]">
                                <div class="flex items-center gap-2 px-2 text-[#d9dbe0]">
                                    <div class="text-xs font-semibold">"Settings"</div>
                                </div>
                            </TitleBar>
                            <div class="flex-1 w-full relative">
                                <SettingsMenu />
                            </div>
                        </div>
                    }
                }, cx).ok();
            } }}>
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
pub fn app_sidebar(collapsed: bool, active_tab: AppTab, set_active_tab: Setter<AppTab>) {
    view! {
        <Sidebar collapsed={collapsed} collapsible={SidebarCollapsible::Icon}
            class="w-full h-full border-r border-[#272a2f]"
            footer={view! {
                <SidebarFooter>
                    <UserSection />
                </SidebarFooter>
            }}
        >
            <SidebarGroup label="">
                <SidebarMenu>
                    <SidebarMenuItem icon={Icon::default().path("icons/dashboard.svg")} label="Dashboard" active={active_tab == AppTab::Dashboard} on_click={move |_, _, cx| set_active_tab(AppTab::Dashboard)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/target.svg")} label="Target" active={active_tab == AppTab::Target} on_click={move |_, _, cx| set_active_tab(AppTab::Target)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/proxy.svg")} label="Proxy" active={active_tab == AppTab::Proxy} on_click={move |_, _, cx| set_active_tab(AppTab::Proxy)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/intruder.svg")} label="Intruder" active={active_tab == AppTab::Intruder} on_click={move |_, _, cx| set_active_tab(AppTab::Intruder)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/repeater.svg")} label="Repeater" active={active_tab == AppTab::Repeater} on_click={move |_, _, cx| set_active_tab(AppTab::Repeater)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/decoder.svg")} label="Decoder" active={active_tab == AppTab::Decoder} on_click={move |_, _, cx| set_active_tab(AppTab::Decoder)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/comparer.svg")} label="Comparer" active={active_tab == AppTab::Comparer} on_click={move |_, _, cx| set_active_tab(AppTab::Comparer)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/logger.svg")} label="Logger" active={active_tab == AppTab::Logger} on_click={move |_, _, cx| set_active_tab(AppTab::Logger)} />
                    <SidebarMenuItem icon={IconName::Folder} label="Organizer" active={active_tab == AppTab::Organizer} on_click={move |_, _, cx| set_active_tab(AppTab::Organizer)} />
                    <SidebarMenuItem icon={Icon::default().path("icons/extension.svg")} label="Extensions" active={active_tab == AppTab::Extensions} on_click={move |_, _, cx| set_active_tab(AppTab::Extensions)} />
                </SidebarMenu>
            </SidebarGroup>
        </Sidebar>
    }
}


