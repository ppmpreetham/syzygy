use gpui_kit::component::Sizable;
use gpui_kit::component::button::*;
use gpui_kit::component::sidebar::*;
use gpui_kit::component::Icon;
use gpui_kit::{assets::IconName, prelude::FluentBuilder};
use gpui_kit::*;
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
pub fn app_sidebar(collapsed: bool) {
    view! {
        <Sidebar collapsed={collapsed}
            class="w-full h-full bg-[#141517] border-r border-[#272a2f]"
            footer={view! {
                <SidebarFooter>
                    <UserSection />
                </SidebarFooter>
            }}
        >
            <SidebarGroup label="">
                <SidebarMenu>
                    <SidebarMenuItem icon={IconName::Inbox} label="Dashboard" active={true} />
                    <SidebarMenuItem icon={IconName::User} label="Target" />
                    <SidebarMenuItem icon={IconName::CircleX} label="Proxy" />
                    <SidebarMenuItem icon={IconName::FolderOpen} label="Intruder" />
                    <SidebarMenuItem icon={IconName::PanelLeft} label="Repeater" />
                    <SidebarMenuItem icon={IconName::PanelLeft} label="Decoder" />
                    <SidebarMenuItem icon={IconName::PanelLeft} label="Comparer" />
                    <SidebarMenuItem icon={IconName::PanelLeft} label="Logger" />
                    <SidebarMenuItem icon={IconName::PanelLeft} label="Organizer" />
                    <SidebarMenuItem icon={IconName::PanelLeft} label="Extensions" />
                </SidebarMenu>
            </SidebarGroup>
        </Sidebar>
    }
}
