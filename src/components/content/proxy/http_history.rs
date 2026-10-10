use crate::backend::ProxyState;
use crate::backend::intercept::state::ProxyEvent;
use crate::backend::uimeta::RequestRow;
use crate::components;
use crate::components::content::intruder;
use crate::globals::FULL_ROW_RAM;
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::table::TableState;
use gpui_kit::gpui::SharedString;
use gpui_kit::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;
use zopra::components::declarative_table::DeclarativeTableDelegate;
use zopra::hooks::Setter;
use zopra::{component, hooks::use_state, view};

type HistoryRow = (
    usize,
    RequestRow,
    Option<(u32, u32)>,
    Setter<HashMap<usize, (u32, u32)>>,
);

fn row_time(row: &RequestRow) -> String {
    row.start_response_timer
        .zip(row.end_response_timer)
        .and_then(|(start, end)| end.duration_since(start).ok())
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_default()
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

struct HighlightedCell {
    value: SharedString,
    colors: Option<(u32, u32)>,
}

impl PartialEq for HighlightedCell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for HighlightedCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl IntoElement for HighlightedCell {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut cell = div().w_full().h_full();
        if let Some((background, foreground)) = self.colors {
            cell.style().background = Some(gpui::Fill::Color(gpui::rgba(background).into()));
            cell = cell.text_color(gpui::rgba(foreground));
        }
        cell.child(self.value)
    }
}

fn highlighted_cell(row: &HistoryRow, value: impl Into<SharedString>) -> HighlightedCell {
    HighlightedCell {
        value: value.into(),
        colors: row.2,
    }
}

#[component]
pub fn shortcut_item(name: String, shortcut: String) {
    view! {
        <div class="flex flex-row w-full justify-between">
            { name }
            <div class="text-sm text-[#888888]">{ shortcut }</div>
        </div>
    }
}

#[allow(clippy::too_many_lines)]
#[component]
pub fn http_history(
    proxy_state: Arc<ProxyState>,
    set_app_tab: Setter<components::AppTab>,
    set_intruders: Setter<Vec<intruder::state::IntruderState>>,
    set_intruder_tab: Setter<usize>,
) {
    let (get_highlights, set_highlights) = use_state(HashMap::<usize, (u32, u32)>::new());
    let (get_visible, set_visible) = use_state(0..1000usize);
    let (get_total, set_total) = use_state(0usize);
    let (started, set_started) = use_state(false);

    if !*started {
        set_started(true);
        let state = Arc::clone(&proxy_state);
        let set_tot = set_total.clone();
        window
            .spawn(cx, move |cx_ref: &mut AsyncWindowContext| {
                let mut async_cx = (*cx_ref).clone();
                async move {
                    let mut rx = state.subscribe();
                    let cnt = state.history_count();
                    async_cx.update(|_, cx| set_tot(cnt)).ok();
                    while let Ok(event) = rx.recv().await {
                        if matches!(event, ProxyEvent::History(_, _)) {
                            let cnt = state.history_count();
                            async_cx.update(|_, cx| set_tot(cnt)).ok();
                        }
                    }
                }
            })
            .detach();
    }
    let highlights = get_highlights(cx);
    let visible = get_visible(cx);
    let total = get_total(cx);

    let requests = if FULL_ROW_RAM {
        proxy_state.history_rows()
    } else {
        proxy_state.history_slice((*visible).clone())
    };
    let rows = requests
        .iter()
        .map(|(history_idx, row)| {
            (
                *history_idx,
                row.clone(),
                highlights.get(history_idx).copied(),
                set_highlights.clone(),
            )
        })
        .collect::<Vec<_>>();
    let current_range = (*visible).clone();
    view! {
        <div class="flex flex-col size-full text-[#ededed] w-auto">
            <DataTable
                rows={rows}
                rows_count={if FULL_ROW_RAM { requests.len() } else { *total }}
                data_offset={if FULL_ROW_RAM { 0 } else { visible.start }}
                on_visible_rows_changed={{
                    let current_range = current_range.clone();
                    let set_visible = set_visible.clone();
                    move |range: Range<usize>, cx: &mut gpui_kit::App| {
                        if !FULL_ROW_RAM && range != current_range {
                          set_visible.set(range, cx);
                        }
                    }
                }}
                row_selectable={true}
                cell_selectable={false}
                col_selectable={false}
                loop_selection={true}
                row_header={true}
                on_context_menu={{
                    let set_app_tab = set_app_tab.clone();
                    let set_intruders = set_intruders.clone();
                    move |req, row_ix, menu, window, cx| {
                        history_context_menu(req, row_ix, menu, window, cx, set_app_tab.clone(), set_intruders.clone(), set_intruder_tab.clone())
                    }
                }}
            >
                <Col id="id" title="#" width={50.} r={|req| highlighted_cell(req, req.0.to_string())} />
                <Col id="host" title="Host" width={150.} r={|req| highlighted_cell(req, req.1.host.clone())} sortable />
                <Col id="method" title="Method" width={80.} r={|req| highlighted_cell(req, req.1.method.to_string())} sortable />
                <Col id="url" title="URL" r={|req| highlighted_cell(req, req.1.uri.to_string())} sortable />
                <Col id="status" title="Status" width={80.} r={|req| highlighted_cell(req, req.1.status.map(|status| status.as_u16().to_string()).unwrap_or_default())} sortable />
                <Col id="length" title="Length" width={80.} r={|req| highlighted_cell(req, req.1.length.to_string())} sortable text_right />
                <Col id="time" title="Time" width={80.} r={|req| highlighted_cell(req, row_time(&req.1))} sortable text_right />
            </DataTable>
        </div>
    }
}

// on rigth click
fn history_context_menu(
    req: &HistoryRow,
    _row_ix: usize,
    menu: PopupMenu,
    window: &mut Window,
    cx: &mut Context<TableState<DeclarativeTableDelegate<HistoryRow>>>,
    set_app_tab: Setter<components::AppTab>,
    set_intruders: Setter<Vec<intruder::state::IntruderState>>,
    set_intruder_tab: Setter<usize>,
) -> PopupMenu {
    let url = format!("https://{}{}", req.1.host, req.1.uri);
    let history_idx = req.0;
    let set_highlights = req.3.clone();
    let highlight_menu = PopupMenu::build(window, cx, move |mut menu, _, _| {
        for (name, background, foreground) in COLORS {
            let color_name = name.to_string();
            let set_highlights = set_highlights.clone();
            menu = menu.item(
            PopupMenuItem::element(move |_, _| {
                view! {
                    <div
                        class="flex flex-row w-full min-w-[150px] h-full items-center text-black bg-white"
                        bg={gpui::rgba(background)}
                        textColor={gpui::rgba(foreground)}
                    >
                        { color_name.clone() }
                    </div>
                }
                .into_any_element()
            })
            .on_click(move |_, _, cx| {
                set_highlights.update(
                    |highlights| {
                        highlights.insert(history_idx, (background, foreground));
                    },
                    cx,
                );
            }),
        );
        }
        let set_highlights = set_highlights.clone();
        menu.item(PopupMenuItem::separator())
            .item(
                PopupMenuItem::new("Clear highlight").on_click(move |_, _, cx| {
                    set_highlights.update(
                        |highlights| {
                            highlights.remove(&history_idx);
                        },
                        cx,
                    );
                }),
            )
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
    menu.item(PopupMenuItem::element(move |_, _| {
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
    .item(PopupMenuItem::submenu(
        "Don't intercept requests",
        dont_intercept_menu,
    ))
    .item(PopupMenuItem::submenu("Do intercept", do_intercept_menu))
    .item(PopupMenuItem::separator())
    .item(PopupMenuItem::new("Scan").disabled(true))
    .item(PopupMenuItem::separator())
    .item(
        PopupMenuItem::element(|window, cx| {
            view! {
            <ShortcutItem name={"Send to Intruder".to_string()} shortcut={"Ctrl+I".to_string()} />
        }.into_any_element()
        })
        .on_click({
            let req_row = req.1.clone();
            let set_intruders = set_intruders.clone();
            let set_app_tab = set_app_tab.clone();

            let set_intruder_tab = set_intruder_tab.clone();
            move |_, _, cx| {
                let raw_req = format!(
                    "{} {} HTTP/1.1\r\nHost: {}\r\n\r\n",
                    req_row.method, req_row.uri, req_row.host
                );
                let mut new_idx = 0;
                set_intruders.update(
                    |list| {
                        list.push(intruder::state::IntruderState {
                            req_id: list.len() + 1,
                            target: req_row.host.clone(),
                            raw_request: raw_req,
                            update_host_header: true,
                            rows: Vec::new(),
                        });
                        new_idx = list.len() - 1;
                    },
                    cx,
                );
                set_intruder_tab.set(new_idx, cx);
                set_app_tab.set(components::AppTab::Intruder, cx);
            }
        }),
    )
    .item(
        PopupMenuItem::element(|window, cx| {
            view! {
            <ShortcutItem name={"Send to Repeater".to_string()} shortcut={"Ctrl+R".to_string()} />
        }.into_any_element()
        })
        .on_click(|_, _, _| println!("Repeater")),
    )
    .item(PopupMenuItem::new("Send to Sequencer"))
    .item(
        PopupMenuItem::element(|window, cx| {
            view! {
            <ShortcutItem name={"Send to Organizer".to_string()} shortcut={"Ctrl+O".to_string()} />
        }.into_any_element()
        })
        .on_click(|_, _, _| println!("Organizer")),
    )
    .item(PopupMenuItem::new("Send to Comparer"))
    .item(PopupMenuItem::submenu("Request in browser", browser_menu))
}
