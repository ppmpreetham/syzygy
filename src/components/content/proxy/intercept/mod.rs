use gpui_kit::component::{menu, menu::PopupMenuItem};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use zopra::{component, hooks::use_state, view};

use crate::backend::ProxyState;
use crate::backend::intercept::parse::parse_request;
use crate::backend::intercept::state::ProxyEvent;
use crate::backend::uimeta::RequestRow;
use std::sync::Arc;

mod browser;
mod inspector;
mod models;
pub mod request_info;
mod request_table;

use browser::open_browser;
use inspector::Inspector;
use models::{DropAction, ForwardAction};
use request_info::RequestInfo;
use request_table::RequestTable;

#[component]
pub fn intercept(proxy_state: Arc<ProxyState>) {
    let (get_requests, set_requests) = use_state(Vec::<(usize, RequestRow)>::new());
    let (get_loop, set_loop) = use_state(false);

    if !*get_loop {
        set_loop(true);
        let state = Arc::clone(&proxy_state);
        let set_reqs = set_requests.clone();

        window
            .spawn(cx, move |cx_ref: &mut AsyncWindowContext| {
                let mut async_cx = (*cx_ref).clone();
                async move {
                    let initial = state.pending_rows();
                    if !initial.is_empty() {
                        async_cx
                            .update(|_, cx| {
                                set_reqs(initial);
                            })
                            .ok();
                    }

                    let mut rx = state.subscribe();
                    while let Ok(event) = rx.recv().await {
                        async_cx
                            .update(|_, cx| {
                                if let ProxyEvent::Intercepted(id, row) = event {
                                    set_reqs(|requests| requests.push((id, row)));
                                }
                            })
                            .ok();
                    }
                }
            })
            .detach();
    }

    let requests = get_requests.clone();
    let (selected_idx, set_selected_idx) = use_state(None::<usize>);

    let selected = (*selected_idx).and_then(|i| requests.get(i));
    let selected_id = selected.map(|request| request.0);

    let req_content = selected
        .and_then(|request| proxy_state.pending_request_text(request.0))
        .unwrap_or_else(|| "No request selected".to_string());

    let req_host = selected
        .map(|request| request.1.host.clone())
        .unwrap_or_default();

    let (forward_action, set_forward_action) = use_state(ForwardAction::Forward);
    let (drop_action, set_drop_action) = use_state(DropAction::Drop);
    let (intercept_state, set_intercept_state) = use_state(proxy_state.intercept_enabled());
    let (editor_entity, set_editor_entity) =
        use_state(None::<Entity<gpui_kit::component::input::EditorState>>);

    let build_fwd_menu = {
        let set_fwd = set_forward_action.clone();
        move |menu: menu::PopupMenu,
              _: &mut gpui_kit::Window,
              _: &mut gpui_kit::Context<menu::PopupMenu>| {
            let s1 = set_fwd.clone();
            let s2 = set_fwd.clone();
            menu.item(
                PopupMenuItem::new("Forward")
                    .on_click(move |_, _, cx| s1.set(ForwardAction::Forward, cx)),
            )
            .item(
                PopupMenuItem::new("Forward All")
                    .on_click(move |_, _, cx| s2.set(ForwardAction::ForwardAll, cx)),
            )
        }
    };

    let build_drp_menu = {
        let set_drp = set_drop_action.clone();
        move |menu: menu::PopupMenu,
              _: &mut gpui_kit::Window,
              _: &mut gpui_kit::Context<menu::PopupMenu>| {
            let s1 = set_drp.clone();
            let s2 = set_drp.clone();
            menu.item(
                PopupMenuItem::new("Drop").on_click(move |_, _, cx| s1.set(DropAction::Drop, cx)),
            )
            .item(
                PopupMenuItem::new("Drop All")
                    .on_click(move |_, _, cx| s2.set(DropAction::DropAll, cx)),
            )
        }
    };

    view! {
      <div class="flex flex-col size-full justify-start text-[#ededed]">
        <div class="flex flex-row p-4 justify-between w-full items-center h-fit shrink-0">
          <div class="flex flex-row gap-2 items-center">
            <button
              id="intercept-on-btn"
              label={if *intercept_state { "Intercept On" } else { "Intercept Off" }}
              on_click={{
              let state = Arc::clone(&proxy_state);
              let set_requests = set_requests.clone();
              let set_intercept_state = set_intercept_state.clone();
              move |_, _, cx| {
                let enabled = !*set_intercept_state.current();
                state.set_intercept(enabled);
                if !enabled {
                  set_requests.update(Vec::clear, cx);
                }
                set_intercept_state(enabled);
                }
              }}
              class={if *intercept_state { "bg-white text-black" } else { "bg-black text-white" }}
            />
            <DropdownButton
              id="forward-dropdown"
              button={view! { <button id="forward-btn" label={match *forward_action {
                ForwardAction::Forward => "Forward",
                ForwardAction::ForwardAll => "Forward All",
              }} on_click={{
                let state = Arc::clone(&proxy_state);
                let set_requests = set_requests.clone();
                let set_intercept_state = set_intercept_state.clone();
                move |_, _, cx| {
                  match *set_forward_action.current() {
                      ForwardAction::Forward => if let Some(id) = selected_id {
                        if let Some(edited) = editor_entity.as_ref().and_then(|editor| {
                            parse_request(editor.read(cx).value().as_ref())
                        }) {
                            set_requests(|requests| requests.retain(|request| request.0 != id));
                            state.forward(id, Some(edited));
                        }
                      },
                      ForwardAction::ForwardAll => {
                        set_requests.update(Vec::clear, cx);
                        state.forward_all();
                        set_intercept_state(false);
                      },
                  }
                }
              }} /> }}
              dropdown_menu={build_fwd_menu}
            />
            <DropdownButton
              id="drop-dropdown"
              button={view! { <button id="drop-btn" label={match *drop_action {
                DropAction::Drop => "Drop",
                DropAction::DropAll => "Drop All",
              }} on_click={{
                let state = Arc::clone(&proxy_state);
                let set_requests = set_requests.clone();
                let set_intercept_state = set_intercept_state.clone();
                move |_, _, cx| {
                  match *set_drop_action.current() {
                      DropAction::Drop => if let Some(id) = selected_id {
                        set_requests(|requests| requests.retain(|request| request.0 != id));
                        state.drop_request(id);
                      },
                      DropAction::DropAll => {
                        set_requests.update(Vec::clear, cx);
                        state.drop_all();
                        set_intercept_state(false);
                      },
                  }
                }
              }} /> }}
              dropdown_menu={build_drp_menu}
            />
          </div>
          <div class="flex flex-row gap-2 items-center">
            <div when={!req_host.is_empty()}>{format!("Request to {}", req_host)}</div>
            <button
              id="open-browser"
              label="Open Browser"
              on_click={move |_, window, cx| open_browser(window, cx)}
            />
            <div>"hamburger"</div>
          </div>
        </div>

        // Main content
        <Resizable id="intercept" vertical>
          <ResizablePanel>
            <div class="flex flex-col size-full bg-[#141517]">
              <RequestTable
                requests={requests.clone()}
                set_selected_idx={set_selected_idx.clone()}
              />
            </div>
          </ResizablePanel>
          <ResizablePanel>
            <Resizable id="intercept-req-info" horizontal>
              <ResizablePanel>
                <RequestInfo content={req_content} set_editor_entity={set_editor_entity.clone()} />
              </ResizablePanel>
              <ResizablePanel>
                <Inspector request={"Hello".to_string()} />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>
        </Resizable>
      </div>
    }
}
