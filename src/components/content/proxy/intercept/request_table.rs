use gpui_kit::prelude::*;
use gpui_kit::*;
use zopra::{
    component,
    hooks::{Setter, Snap},
    view,
};

use crate::backend::uimeta::{Dirn, RequestRow};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

type InterceptRow = (usize, RequestRow, usize, Setter<Option<usize>>);

fn row_time(time: SystemTime) -> String {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| {
            let seconds = duration.as_secs() % 86_400;
            format!(
                "{:02}:{:02}:{:02}",
                seconds / 3_600,
                seconds / 60 % 60,
                seconds % 60
            )
        })
        .unwrap_or_default()
}

struct SelectableCell {
    value: String,
    selected_idx: usize,
    set_selected_idx: Setter<Option<usize>>,
}

impl PartialEq for SelectableCell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for SelectableCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl IntoElement for SelectableCell {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let Self {
            value,
            selected_idx,
            set_selected_idx,
        } = self;
        div()
            .w_full()
            .h_full()
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                set_selected_idx.set(Some(selected_idx), cx);
            })
            .child(value)
    }
}

#[component]
pub(super) fn request_table(
    requests: Snap<Vec<(usize, RequestRow)>>,
    set_selected_idx: Setter<Option<usize>>,
) {
    let rows = requests
        .iter()
        .enumerate()
        .map(|(selected_idx, (request_id, row))| {
            (
                *request_id,
                row.clone(),
                selected_idx,
                set_selected_idx.clone(),
            )
        })
        .collect::<Vec<_>>();
    view! {
        <DataTable
            rows={rows}
            row_selectable={true}
            cell_selectable={false}
            col_selectable={false}
            loop_selection={true}
            row_header={true}
        >
            <Col id="time" title="Time" width={80.} r={|req| selectable_cell(req, row_time(req.1.time))} sortable />
            <Col id="req_type" title="Type" width={80.} r={|req| selectable_cell(req, format!("{:?}", req.1.addr_type))} sortable />
            <Col id="direction" title="Dir" width={80.} r={|req| selectable_cell(req, if req.1.dirn == Dirn::In { "In".to_string() } else { "Out".to_string() })} sortable />
            <Col id="method" title="Method" width={80.} r={|req| selectable_cell(req, req.1.method.to_string())} sortable />
            <Col id="url" title="URL" r={|req| selectable_cell(req, req.1.uri.to_string())} sortable />
            <Col id="status" title="Status" width={80.} r={|req| selectable_cell(req, req.1.status.map(|status| status.as_u16().to_string()).unwrap_or_default())} sortable />
            <Col id="length" title="Length" width={80.} r={|req| selectable_cell(req, req.1.length.to_string())} sortable text_right />
        </DataTable>
    }
}

fn selectable_cell(row: &InterceptRow, value: String) -> SelectableCell {
    SelectableCell {
        value,
        selected_idx: row.2,
        set_selected_idx: row.3.clone(),
    }
}
