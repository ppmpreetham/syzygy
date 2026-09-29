use gpui_kit::component::resizable::*;
use gpui_kit::*;
use strum::FromRepr;
use zopra::{component, hooks::use_state, view};
use gpui_kit::{prelude::FluentBuilder};

#[derive(Clone)]
pub struct InterceptCell {
    pub time: usize,
    // get the request type (GET, POST, etc.) when wiring up
    pub req_type: usize,
    // true if the request is incoming, false if outgoing
    pub direction: bool,
    // the request method (GET, POST, etc.)
    pub method: String,
    pub url: String,
    pub status: u16,
    pub length: usize,
    pub highlight: Option<(u32, u32)>,
}

use gpui_kit::component::button::{Button, DropdownButton};
use gpui_kit::component::menu::PopupMenuItem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub enum ForwardAction {
    Forward = 0,
    ForwardAll = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub enum DropAction {
    Drop = 0,
    DropAll = 1,
}

#[component]
pub fn intercept() {
    let (forward_action, set_forward_action) = use_state(ForwardAction::Forward);
    let (drop_action, set_drop_action) = use_state(DropAction::Drop);

    let fwd_val = forward_action();
    let drp_val = drop_action();

    let fwd_label = match fwd_val {
        ForwardAction::Forward => "Forward",
        ForwardAction::ForwardAll => "Forward All",
    };

    let drp_label = match drp_val {
        DropAction::Drop => "Drop",
        DropAction::DropAll => "Drop All",
    };

    let build_fwd_menu = {
        let set_forward = set_forward_action.clone();
        move |menu: gpui_kit::component::menu::PopupMenu, _window: &mut gpui_kit::Window, _cx: &mut gpui_kit::Context<gpui_kit::component::menu::PopupMenu>| {
            let set_1 = set_forward.clone();
            let set_2 = set_forward.clone();
            menu.item(
                    PopupMenuItem::new("Forward")
                        .on_click(move |_, _, cx| set_1(ForwardAction::Forward, cx))
                )
                .item(
                    PopupMenuItem::new("Forward All")
                        .on_click(move |_, _, cx| set_2(ForwardAction::ForwardAll, cx))
                )
        }
    };

    let build_drp_menu = {
        let set_drop = set_drop_action.clone();
        move |menu: gpui_kit::component::menu::PopupMenu, _window: &mut gpui_kit::Window, _cx: &mut gpui_kit::Context<gpui_kit::component::menu::PopupMenu>| {
            let set_1 = set_drop.clone();
            let set_2 = set_drop.clone();
            menu.item(
                    PopupMenuItem::new("Drop")
                        .on_click(move |_, _, cx| set_1(DropAction::Drop, cx))
                )
                .item(
                    PopupMenuItem::new("Drop All")
                        .on_click(move |_, _, cx| set_2(DropAction::DropAll, cx))
                )
        }
    };

    view! {
        <div class="flex flex-col size-full items-center justify-start text-[#ededed]">

          <div class="flex flex-row p-4 justify-between w-full items-center h-fit shrink-0">
              <div class="flex flex-row gap-2 items-center">
                  <button id="intercept-on-btn" label="Intercept On" />
                  <DropdownButton
                      id="forward-dropdown"
                      button={view! {
                          <button
                              id="forward-btn"
                              label={fwd_label}
                              on_click={move |_, _, _| println!("Executed: {:?}", fwd_val)}
                          />
                      }}
                      dropdown_menu={build_fwd_menu}
                  />
                  <DropdownButton
                      id="drop-dropdown"
                      button={view! {
                          <button
                              id="drop-btn"
                              label={drp_label}
                              on_click={move |_, _, _| println!("Executed: {:?}", drp_val)}
                          />
                      }}
                      dropdown_menu={build_drp_menu}
                  />
              </div>
              <div class="flex flex-row gap-2 items-center">
                  <div>"Request to https://smtg.com:smtg [smtg:smtg:smtg:smtg]"</div>
                  <div>"Open Browser"</div>
                  <div>"hamburger"</div>
              </div>
          </div>

          <Resizable id="intercept" vertical>
            <ResizablePanel>
              <div class="flex flex-col p-4 size-full items-center justify-center">
                <div class="text-gray-500">"Request Viewer Placeholder"</div>
              </div>
            </ResizablePanel>
            <ResizablePanel>
              <Resizable id="intercept-req-info" horizontal>
                <ResizablePanel>
                  <div class="flex flex-col p-4">
                    <div>"Request Info"</div>
                  </div>
                </ResizablePanel>
                <ResizablePanel>
                  <Inspector request={String::from("Hello")} />
                </ResizablePanel>
              </Resizable>
            </ResizablePanel>
          </Resizable>

        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(usize)]
enum InspectorTab{
  Inspect = 0,
  Notes = 1,
}

// later change the input to RequestInfo
#[component]
pub fn inspector(request: String){
  let (active_tab, set_active_tab) = use_state(InspectorTab::Inspect);
  let tab_val = active_tab();

  view! {
      <div class="flex flex-col size-full bg-[#141517]">
          <nav
              underline
              selected_index={tab_val as usize}
              on_click={move |index, _, cx| {
                  if let Some(tab) = InspectorTab::from_repr(*index) {
                      set_active_tab(tab, cx);
                  }
              }}
              class="px-4"
          >
              <Tab label="Inspector" />
              <Tab label="Notes" />
          </nav>

          <div class="flex-1 w-full text-[#ededed]">
              {match tab_val {
                  _ => view! { <div></div> }.into_any_element(),
              }}
          </div>
      </div>
  }
}
