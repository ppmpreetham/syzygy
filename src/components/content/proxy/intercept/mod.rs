use crate::backend::ProxyState;
use crate::backend::intercept::parse::parse_request;
use crate::backend::intercept::state::ProxyEvent;
use crate::backend::uimeta::RequestRow;
use crate::components::common::tree_sitter;
use component::input::EditorState;
use gpui_kit::component;
use gpui_kit::component::{menu, menu::PopupMenuItem};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::sync::Arc;
use zopra::ui::{DropdownButton, Resizable, ResizablePanel};
use zopra::{
    component,
    hooks::{Signal, use_signal, use_state},
    use_effect, view,
};
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

use gpui_kit::component::highlighter::{GrammarConfig, LanguageRegistry};
use std::sync::Once;

static INIT_GRAMMAR: Once = Once::new();

#[component]
pub fn intercept(proxy_state: Arc<ProxyState>) {
    INIT_GRAMMAR.call_once(|| {
        LanguageRegistry::singleton().register(
            "http",
            &GrammarConfig::new(
                "http",
                tree_sitter::LANGUAGE.clone(),
                vec![],
                include_str!("queries/highlights.scm"),
                include_str!("queries/injections.scm"),
                "",
            ),
        );
    });
    let (requests, set_requests) = use_state(Vec::<(usize, RequestRow)>::new());

    use_effect!(
        || {
            cx.spawn(async move |cx| {
                let initial = proxy_state.pending_rows();
                cx.update(|cx| set_requests(initial));

                let mut rx = proxy_state.subscribe();
                while let Ok(event) = rx.recv().await {
                    if let ProxyEvent::Intercepted(id, row) = event
                        && cx.update(|cx| set_requests.try_update(|requests| requests.push((id, row)))).is_none()
                    {
                        break; // the component unmounted
                    }
                }
            })
            .detach();
        },
        []
    );

    let selected = use_signal(None::<usize>);
    let selected_request = selected()
        .and_then(|index| requests.read().get(index))
        .map(|(id, row)| (*id, row.host.clone()));
    let selected_id = selected_request.as_ref().map(|(id, _)| *id);

    let (content, set_content) = use_state("No request selected");
    let (host, set_host) = use_state(SharedString::default());
    use_effect!(
        move || {
            set_content(
                selected_id
                    .and_then(|id| proxy_state.pending_request_text(id))
                    .map_or_else(|| "No request selected".into(), Into::into),
            );
            set_host(selected_request.map(|(_, host)| host).unwrap_or_default());
        },
        [selected_id]
    );

    let forward_action = use_signal(ForwardAction::Forward);
    let drop_action = use_signal(DropAction::Drop);
    let intercepting = use_signal(proxy_state.intercept_enabled());
    let request_editor = use_signal(None::<Entity<EditorState>>);

    view! {
      <div class="flex flex-col size-full justify-start text-[#ededed]">
        <div class="flex flex-row p-4 justify-between w-full items-center h-fit shrink-0">
          <div class="flex flex-row gap-2 items-center">
            <button
              id="intercept-on-btn"
              label={if intercepting() { "Intercept On" } else { "Intercept Off" }}
              on_click={|| {
                let enabled = !intercepting();
                proxy_state.set_intercept(enabled);
                if !enabled {
                  set_requests(|requests| requests.clear());
                }
                intercepting(enabled);
              }}
              class={if intercepting() { "bg-theme-primary text-black" } else { "bg-black text-white" }}
            />
            <DropdownButton
              id="forward-dropdown"
              button={view! { <button id="forward-btn" label={match forward_action() {
                ForwardAction::Forward => "Forward",
                ForwardAction::ForwardAll => "Forward All",
              }} on_click={|_, _, cx| match forward_action() {
                ForwardAction::Forward => if let Some(id) = selected_id
                  && let Some(edited) = request_editor
                    .read()
                    .as_ref()
                    .and_then(|editor| parse_request(editor.read(cx).value().as_ref()))
                {
                  set_requests(|requests| requests.retain(|request| request.0 != id));
                  proxy_state.forward(id, Some(edited));
                },
                ForwardAction::ForwardAll => {
                  set_requests(|requests| requests.clear());
                  proxy_state.forward_all();
                  intercepting(false);
                },
              }} /> }}
              dropdown_menu={action_menu(forward_action, [
                ("Forward", ForwardAction::Forward),
                ("Forward All", ForwardAction::ForwardAll),
              ])}
            />
            <DropdownButton
              id="drop-dropdown"
              button={view! { <button id="drop-btn" label={match drop_action() {
                DropAction::Drop => "Drop",
                DropAction::DropAll => "Drop All",
              }} on_click={|| match drop_action() {
                DropAction::Drop => if let Some(id) = selected_id {
                  set_requests(|requests| requests.retain(|request| request.0 != id));
                  proxy_state.drop_request(id);
                },
                DropAction::DropAll => {
                  set_requests(|requests| requests.clear());
                  proxy_state.drop_all();
                  intercepting(false);
                },
              }} /> }}
              dropdown_menu={action_menu(drop_action, [
                ("Drop", DropAction::Drop),
                ("Drop All", DropAction::DropAll),
              ])}
            />
          </div>
          <div class="flex flex-row gap-2 items-center">
            <div when={!host.read().is_empty()}>{format!("Request to {}", host.read())}</div>
            <button
              id="open-browser"
              label="Open Browser"
              on_click={|_, window, cx| open_browser(window, cx)}
            />
            <div>"hamburger"</div>
          </div>
        </div>

        // Main content
        <Resizable id="intercept" vertical>
          <ResizablePanel>
            <div class="flex flex-col size-full">
              <RequestTable requests={requests} selected={selected} />
            </div>
          </ResizablePanel>
          <ResizablePanel>
            <Resizable id="intercept-req-info" horizontal>
              <ResizablePanel>
                <RequestInfo content={content()} editor_entity={request_editor} />
              </ResizablePanel>
              <ResizablePanel>
                <Inspector request="Hello" />
              </ResizablePanel>
            </Resizable>
          </ResizablePanel>
        </Resizable>
      </div>
    }
}
fn action_menu<T: Clone + PartialEq + 'static, const N: usize>(
    action: Signal<T>,
    actions: [(&'static str, T); N],
) -> impl Fn(menu::PopupMenu, &mut Window, &mut Context<menu::PopupMenu>) -> menu::PopupMenu + 'static {
    move |menu, _, _| {
        actions.iter().cloned().fold(menu, |menu, (label, value)| {
            menu.item(PopupMenuItem::new(label).on_click(move |_, _, cx| action.set(value.clone(), cx)))
        })
    }
}
