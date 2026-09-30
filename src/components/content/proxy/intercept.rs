use gpui_kit::component::button::{Button, DropdownButton};
use gpui_kit::component::menu::PopupMenuItem;
use gpui_kit::component::resizable::*;
use gpui_kit::component::table::DataTable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use strum::FromRepr;
use zopra::cn;
use zopra::{component, hooks::use_state, view};

use crate::backend::webview::proxy_config;

use zopra::components::webview::{WebView, WebViewProps, use_webview};
pub struct BrowserRoot {
  pub webview_ctrl: zopra::components::webview::WebViewController,
  pub input_state: gpui_kit::Entity<gpui_kit::component::input::InputState>,
}

pub fn init_browser(
  webview_ctrl: zopra::components::webview::WebViewController,
  input_state: gpui_kit::Entity<gpui_kit::component::input::InputState>,
  cx: &mut gpui_kit::Context<BrowserRoot>,
) -> BrowserRoot {
  let ctrl = webview_ctrl.clone();
  let input = input_state.clone();
  cx.subscribe(&input_state, move |_, _, event, cx| {
    if let gpui_kit::component::input::InputEvent::PressEnter { .. } = event {
      let text = input.read(cx).text().to_string();
      let url = if !text.starts_with("http") && !text.starts_with("file://") {
        format!("https://{}", text)
      } else {
        text
      };
      ctrl.load_url(&url, cx);
    }
  })
  .detach();
  BrowserRoot {
    webview_ctrl,
    input_state,
  }
}

impl gpui_kit::Render for BrowserRoot {
  fn render(
    &mut self,
    window: &mut gpui_kit::Window,
    cx: &mut gpui_kit::Context<Self>,
  ) -> impl gpui_kit::IntoElement {
    let ctrl = self.webview_ctrl.clone();
    let input_state = self.input_state.clone();
    zopra::view! {
        <div class="flex flex-col size-full p-2 gap-2 ">
            <div class="flex flex-row gap-2 items-center px-2 py-1 rounded shadow-lg bg-[#18181b]">
                <button
                    id="win-btn-back"
                    on_click={{ let c = ctrl.clone(); move |_, _, cx| c.back(cx) }}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={gpui_kit::component::Icon::default().path("icons/browser/caret-left.svg")}
                />
                <button
                    id="win-btn-forward"
                    on_click={{ let c = ctrl.clone(); move |_, _, cx| c.forward(cx) }}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={gpui_kit::component::Icon::default().path("icons/browser/caret-right.svg")}
                />
                <button
                    id="win-btn-reload"
                    on_click={{ let c = ctrl.clone(); move |_, _, cx| c.reload(cx) }}
                    class="bg-[#27272a] text-white px-3 py-1 rounded w-fit"
                    icon={gpui_kit::component::Icon::default().path("icons/browser/refresh.svg")}
                />

                <input state={&input_state} class="text-white"/>

                <button id="win-btn-go" on_click={{
                    let c = ctrl.clone();
                    let input = input_state.clone();
                    move |_, _, cx| {
                        let text = input.read(cx).text().to_string();
                        let url = if !text.starts_with("http") {
                            format!("https://{}", text)
                        } else {
                            text
                        };
                        c.load_url(&url, cx);
                    }
                }} class="bg-blue-600  text-white px-4 py-1 rounded">"Go"</button>
            </div>
            <div class="size-full flex-1 relative rounded overflow-hidden">
                <WebView
                    url={"https://google.com".to_string()}
                    controller={Some(ctrl)}
                    devtools={Some(true)}
                    transparent={Some(true)}
                    proxy={Some(proxy_config())}
                />
            </div>
        </div>
    }
  }
}

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
  let (get_requests, _set_requests) = use_state(vec![
    InterceptCell {
      time: 50,
      req_type: 1,
      direction: true,
      method: "GET".to_string(),
      url: "/test".to_string(),
      status: 200,
      length: 1024,
      highlight: None,
    },
    InterceptCell {
      time: 120,
      req_type: 1,
      direction: false,
      method: "POST".to_string(),
      url: "/login".to_string(),
      status: 401,
      length: 256,
      highlight: None,
    },
  ]);
  let requests = get_requests();

  let (forward_action, set_forward_action) = use_state(ForwardAction::Forward);
  let (drop_action, set_drop_action) = use_state(DropAction::Drop);
  let (intercept_state, set_intercept_state) = use_state(false);
  let icpt_state = intercept_state();
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
    move |menu: gpui_kit::component::menu::PopupMenu,
          _window: &mut gpui_kit::Window,
          _cx: &mut gpui_kit::Context<gpui_kit::component::menu::PopupMenu>| {
      let set_1 = set_forward.clone();
      let set_2 = set_forward.clone();
      menu
        .item(
          PopupMenuItem::new("Forward").on_click(move |_, _, cx| set_1(ForwardAction::Forward, cx)),
        )
        .item(
          PopupMenuItem::new("Forward All")
            .on_click(move |_, _, cx| set_2(ForwardAction::ForwardAll, cx)),
        )
    }
  };

  let build_drp_menu = {
    let set_drop = set_drop_action.clone();
    move |menu: gpui_kit::component::menu::PopupMenu,
          _window: &mut gpui_kit::Window,
          _cx: &mut gpui_kit::Context<gpui_kit::component::menu::PopupMenu>| {
      let set_1 = set_drop.clone();
      let set_2 = set_drop.clone();
      menu
        .item(PopupMenuItem::new("Drop").on_click(move |_, _, cx| set_1(DropAction::Drop, cx)))
        .item(
          PopupMenuItem::new("Drop All").on_click(move |_, _, cx| set_2(DropAction::DropAll, cx)),
        )
    }
  };

  view! {
      <div class="flex flex-col size-full justify-start text-[#ededed]">

        <div class="flex flex-row p-4 justify-between w-full items-center h-fit shrink-0">
            <div class="flex flex-row gap-2 items-center">
                <button
                  id="intercept-on-btn"
                  label={if icpt_state { "Intercept On" } else { "Intercept Off" }}
                  on_click={move |_, _, cx| set_intercept_state(!icpt_state, cx)}
                  class={if icpt_state { "bg-white text-black" } else { "bg-black text-white" }}
                />
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
                                <button
                    id="open-browser"
                    label="Open Browser"
                    on_click={move |_, window, cx| {

                                                  let webview_ctrl = zopra::components::webview::WebViewController::new();
                          let input_state = cx.new(|cx| {
                              let mut s = gpui_kit::component::input::InputState::new(window, cx);
                              s.set_value("https://google.com", window, cx);
                              s
                          });
                          let window_options = gpui_kit::WindowOptions {
                              window_bounds: Some(gpui_kit::WindowBounds::Windowed(gpui_kit::Bounds::centered(None, gpui_kit::size(gpui_kit::px(1000.), gpui_kit::px(800.)), cx))),
                              window_background: gpui_kit::gpui::WindowBackgroundAppearance::Transparent,
                              ..Default::default()
                          };
                          cx.open_window(window_options, |window, cx| {
                              window.activate_window();
                              cx.new(|cx| init_browser(webview_ctrl, input_state, cx))
                          }).expect("failed to open browser window");
                    }}
                />
                <div>"hamburger"</div>
            </div>
        </div>

        <Resizable id="intercept" vertical>
          <ResizablePanel>
            <div class="flex flex-col size-full bg-[#141517]">
              <DataTable
                  row_selectable={true}
                  cell_selectable={false}
                  col_selectable={false}
                  loop_selection={true}
                  row_header={true}
              >
                  <thead>
                      <tr>
                          <th id="time" width={80.} sortable>"Time"</th>
                          <th id="req_type" width={80.} sortable>"Type"</th>
                          <th id="direction" width={80.} sortable>"Dir"</th>
                          <th id="method" width={80.} sortable>"Method"</th>
                          <th id="url" sortable>"URL"</th>
                          <th id="status" width={80.} sortable>"Status"</th>
                          <th id="length" width={80.} sortable text_right>"Length"</th>
                      </tr>
                  </thead>
                  <tbody items={requests.clone()}>
                  {|req| view! {
                          <tr class={cn!(
                              "",
                              req.highlight.map(|(bg, text)| format!("bg-[#{:08x}] text-[#{:08x}]", bg, text)).as_deref()
                          )}>
                              <td id="time">{ req.time.to_string() }</td>
                              <td id="req_type">{ req.req_type.to_string() }</td>
                              <td id="direction">{ if req.direction { "In".to_string() } else { "Out".to_string() } }</td>
                              <td id="method">{ req.method.clone() }</td>
                              <td id="url">{ req.url.clone() }</td>
                              <td id="status">{ req.status.to_string() }</td>
                              <td id="length">{ req.length.to_string() }</td>
                          </tr>
                      }}
                  </tbody>
              </DataTable>
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
enum InspectorTab {
  Inspect = 0,
  Notes = 1,
}

// later change the input to RequestInfo
#[component]
pub fn inspector(request: String) {
  let _ = request;
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
