use crate::backend::ProxyState;
use crate::backend::intercept::state::ProxyEvent;
use crate::backend::uimeta::RequestRow;
use crate::components::AppTab;
use crate::components::content::intruder::state::Intruders;
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
use zopra::{
    component,
    hooks::{Signal, use_context, use_signal, use_state},
    use_effect, view,
};

type HistoryRow = (usize, RequestRow, Option<(u32, u32)>);

type Highlights = HashMap<usize, (u32, u32)>;

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

#[component]
pub fn http_history(proxy_state: Arc<ProxyState>) {
    let app_tab: Signal<AppTab> = use_context();
    let intruders: Signal<Intruders> = use_context();
    let highlights = use_signal(Highlights::new());
    let (visible, set_visible) = use_state(0..1000usize);
    let total = use_signal(0usize);

    use_effect!(
        || {
            cx.spawn(async move |cx| {
                let mut rx = proxy_state.subscribe();
                cx.update(|cx| total(proxy_state.history_count()));
                while let Ok(event) = rx.recv().await {
                    if matches!(event, ProxyEvent::History(_, _))
                        && cx.update(|cx| total.try_update(|total| *total = proxy_state.history_count())).is_none()
                    {
                        break; // the component unmounted
                    }
                }
            })
            .detach();
        },
        []
    );

    let range = visible();
    let requests = if FULL_ROW_RAM {
        proxy_state.history_rows()
    } else {
        proxy_state.history_slice(range.clone())
    };
    let rows_count = if FULL_ROW_RAM { requests.len() } else { total() };
    let rows = {
        let highlights = highlights.read();
        requests
            .into_iter()
            .map(|(index, row)| (index, row, highlights.get(&index).copied()))
            .collect::<Vec<HistoryRow>>()
    };

    view! {
        <div class="flex flex-col size-full text-[#ededed] w-auto">
            <DataTable
                rows={rows}
                rows_count={rows_count}
                data_offset={if FULL_ROW_RAM { 0 } else { range.start }}
                on_visible_rows_changed={|range: Range<usize>, cx: &mut App| {
                    if !FULL_ROW_RAM && range != visible() {
                        set_visible(range);
                    }
                }}
                row_selectable={true}
                cell_selectable={false}
                col_selectable={false}
                loop_selection={true}
                row_header={true}
                on_context_menu={|row, _row_ix, menu, window, cx| {
                    history_context_menu(row, menu, window, cx, app_tab, intruders, highlights)
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

// on right click
fn history_context_menu(
    row: &HistoryRow,
    menu: PopupMenu,
    window: &mut Window,
    cx: &mut Context<TableState<DeclarativeTableDelegate<HistoryRow>>>,
    app_tab: Signal<AppTab>,
    intruders: Signal<Intruders>,
    highlights: Signal<Highlights>,
) -> PopupMenu {
    let url = format!("https://{}{}", row.1.host, row.1.uri);
    let index = row.0;
    let highlight_menu = PopupMenu::build(window, cx, move |menu, _, _| {
        COLORS
            .iter()
            .fold(menu, |menu, &(name, background, foreground)| {
                menu.item(
                    PopupMenuItem::element(move |_, _| {
                        view! {
                            <div
                                class="flex flex-row w-full min-w-[150px] h-full items-center text-black bg-white"
                                bg={gpui::rgba(background)}
                                textColor={gpui::rgba(foreground)}
                            >
                                { name }
                            </div>
                        }
                    })
                    .on_click(move |_, _, cx| {
                        highlights.update(|highlights| highlights.insert(index, (background, foreground)), cx);
                    }),
                )
            })
            .item(PopupMenuItem::separator())
            .item(PopupMenuItem::new("Clear highlight").on_click(move |_, _, cx| {
                highlights.update(|highlights| highlights.remove(&index), cx);
            }))
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
            view! { <ShortcutItem name="Send to Intruder" shortcut="Ctrl+I" /> }
        })
        .on_click({
            let request = row.1.clone();
            move |_, _, cx| {
                let raw_request = format!(
                    "{} {} HTTP/1.1\r\nHost: {}\r\n\r\n",
                    request.method, request.uri, request.host
                );
                intruders.update(|intruders| intruders.open(request.host.clone(), raw_request), cx);
                app_tab.set(AppTab::Intruder, cx);
            }
        }),
    )
    .item(
        PopupMenuItem::element(|window, cx| {
            view! { <ShortcutItem name="Send to Repeater" shortcut="Ctrl+R" /> }
        })
        .on_click(|_, _, _| println!("Repeater")),
    )
    .item(PopupMenuItem::new("Send to Sequencer"))
    .item(
        PopupMenuItem::element(|window, cx| {
            view! { <ShortcutItem name="Send to Organizer" shortcut="Ctrl+O" /> }
        })
        .on_click(|_, _, _| println!("Organizer")),
    )
    .item(PopupMenuItem::new("Send to Comparer"))
    .item(PopupMenuItem::submenu("Request in browser", browser_menu))
}
