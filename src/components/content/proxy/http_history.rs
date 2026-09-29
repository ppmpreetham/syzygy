use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::table::{ColumnFixed, DataTable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use zopra::cn;
use zopra::{component, hooks::use_state, view};

#[derive(Clone)]
pub struct RequestCell {
  pub id: usize,
  pub host: String,
  pub method: String,
  pub url: String,
  pub status: u16,
  pub length: usize,
  pub time: usize,
  pub highlight: Option<(u32, u32)>,
}

const COLORS: [(&str, u32, u32); 10] = [
  ("White", 0xffffffff, 0x000000ff),
  ("Red", 0xff6666ff, 0xffffffff),
  ("Orange", 0xffcc66ff, 0x000000ff),
  ("Yellow", 0xffff66ff, 0x000000ff),
  ("Green", 0x66ff66ff, 0x000000ff),
  ("Cyan", 0x66ffffff, 0x000000ff),
  ("Blue", 0x6666ffff, 0xffffffff),
  ("Pink", 0xffb3b3ff, 0x000000ff),
  ("Magenta", 0xff66ffff, 0x000000ff),
  ("Gray", 0x999999ff, 0x000000ff),
];

#[component]
pub fn shortcut_item(name: String, shortcut: String) {
  view! {
      <div class="flex flex-row w-full justify-between">
          { name }
          <div class="text-sm text-[#888888]">{ shortcut }</div>
      </div>
  }
}

#[component]
pub fn http_history() {
  let (get_requests, set_requests) = use_state(vec![
    RequestCell {
      id: 1,
      host: "www.google.com".to_string(),
      method: "GET".to_string(),
      url: "/search?q=test".to_string(),
      status: 200,
      length: 1024,
      time: 50,
      highlight: None,
    },
    RequestCell {
      id: 2,
      host: "api.example.com".to_string(),
      method: "POST".to_string(),
      url: "/login".to_string(),
      status: 401,
      length: 256,
      time: 120,
      highlight: None,
    },
    RequestCell {
      id: 3,
      host: "test.com".to_string(),
      method: "GET".to_string(),
      url: "/style.css".to_string(),
      status: 404,
      length: 0,
      time: 20,
      highlight: None,
    },
  ]);

  let requests = get_requests(cx);

  view! {
      <div class="flex flex-col size-full bg-[#141517] text-[#ededed] w-auto">
          <DataTable
              row_selectable={true}
              cell_selectable={false}
              col_selectable={false}
              loop_selection={true}
              row_header={true}
              on_context_menu={move |req: &RequestCell, _row_ix, menu, window, cx| {
                  let url = format!("https://{}{}", req.host, req.url);
                  let req_id = req.id;

                  let get_reqs = get_requests.clone();
                  let set_reqs = set_requests.clone();

                  let highlight_menu = PopupMenu::build(window, cx, move |mut m, _, _| {
                      for (name, hex, text_hex) in COLORS {
                          let color_name = name.to_string();

                          let get_reqs_loop = get_reqs.clone();
                          let set_reqs_loop = set_reqs.clone();

                          m = m.item(PopupMenuItem::element(move |_, _| {
                              view! {
                                  <div
                                      class="flex flex-row w-full min-w-[150px] h-full items-center text-black bg-white"
                                      bg={gpui::rgba(hex)}
                                      text_color={gpui::rgba(text_hex)}
                                  >
                                      { color_name.clone() }
                                  </div>
                              }
                          }).on_click(move |_, _, cx| {
                              let mut new_reqs = get_reqs_loop(cx);
                              if let Some(r) = new_reqs.iter_mut().find(|r| r.id == req_id) {
                                  r.highlight = Some((hex, text_hex));
                              }
                              set_reqs_loop(new_reqs, cx);
                          }));
                      }

                      let get_reqs_clear = get_reqs.clone();
                      let set_reqs_clear = set_reqs.clone();
                      m = m.item(PopupMenuItem::separator())
                       .item(PopupMenuItem::new("Clear highlight").on_click(move |_, _, cx| {
                          let mut new_reqs = get_reqs_clear(cx);
                          if let Some(r) = new_reqs.iter_mut().find(|r| r.id == req_id) {
                              r.highlight = None;
                          }
                          set_reqs_clear(new_reqs, cx);
                       }));

                      m
                  });

                  let dont_intercept_menu = PopupMenu::build(window, cx, |m, _, _| {
                      m.item(PopupMenuItem::new("To this host"))
                  });

                  let do_intercept_menu = PopupMenu::build(window, cx, |m, _, _| {
                      m.item(PopupMenuItem::new("Responses to this request"))
                  });

                  let browser_menu = PopupMenu::build(window, cx, |m, _, _| {
                      m.item(PopupMenuItem::new("In original session"))
                  });

                  menu
                      .item(PopupMenuItem::element(move |_, _| {
                          view! {
                              <div class="flex flex-row w-full items-center px-2 py-1 font-bold">
                                  { url.clone() }
                              </div>
                          }
                      }))
                      .item(PopupMenuItem::separator())
                      .item(PopupMenuItem::new("Add to scope"))
                      .item(PopupMenuItem::separator())
                      .item(PopupMenuItem::new("Forward"))
                      .item(PopupMenuItem::new("Drop"))
                      .item(PopupMenuItem::separator())
                      .item(PopupMenuItem::new("Add notes"))
                      .item(PopupMenuItem::submenu("Highlight", highlight_menu))
                      .item(PopupMenuItem::separator())
                      .item(PopupMenuItem::submenu("Don't intercept requests", dont_intercept_menu))
                      .item(PopupMenuItem::submenu("Do intercept", do_intercept_menu))
                      .item(PopupMenuItem::separator())
                      .item(PopupMenuItem::new("Scan").disabled(true))
                      .item(PopupMenuItem::separator())
                      .item(PopupMenuItem::element(|window, cx| {
                          view! {
                              <ShortcutItem name={"Send to Intruder".to_string()} shortcut={"Ctrl+I".to_string()} />
                          }.into_any_element()
                      }).on_click(|_, _, _| println!("Intruder")))
                      .item(PopupMenuItem::element(|window, cx| {
                          view! {
                              <ShortcutItem name={"Send to Repeater".to_string()} shortcut={"Ctrl+R".to_string()} />
                          }.into_any_element()
                      }).on_click(|_, _, _| println!("Repeater")))
                      .item(PopupMenuItem::new("Send to Sequencer"))
                      .item(PopupMenuItem::element(|window, cx| {
                          view! {
                              <ShortcutItem name={"Send to Organizer".to_string()} shortcut={"Ctrl+O".to_string()} />
                          }.into_any_element()
                      }).on_click(|_, _, _| println!("Organizer")))
                      .item(PopupMenuItem::new("Send to Comparer"))
                      .item(PopupMenuItem::submenu("Request in browser", browser_menu))
              }}
          >
              <thead>
                  <tr>
                      <th id="id" width={50.}>"#"</th>
                      <th id="host" width={150.} sortable>"Host"</th>
                      <th id="method" width={80.} sortable>"Method"</th>
                      <th id="url" sortable>"URL"</th>
                      <th id="status" width={80.} sortable>"Status"</th>
                      <th id="length" width={80.} sortable text_right>"Length"</th>
                      <th id="time" width={80.} sortable text_right>"Time"</th>
                  </tr>
              </thead>
              <tbody items={requests.clone()}>

              {|req| view! {
                      <tr class={cn!(
                          "",
                          req.highlight.map(|(bg, text)| format!("bg-[#{:08x}] text-[#{:08x}]", bg, text))
                      )}>
                          <td id="id">{ req.id.to_string() }</td>
                          <td id="host">{ req.host.clone() }</td>
                          <td id="method">{ req.method.clone() }</td>
                          <td id="url">{ req.url.clone() }</td>
                          <td id="status">{ req.status.to_string() }</td>
                          <td id="length">{ req.length.to_string() }</td>
                          <td id="time">{ req.time.to_string() }</td>
                      </tr>
                  }}
              </tbody>
          </DataTable>
      </div>
  }
}
