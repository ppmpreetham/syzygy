use gpui_kit::prelude::*;
use zopra::{
    component,
    hooks::Signal,
    view,
};

use crate::backend::uimeta::{Dirn, RequestRow};
use std::time::{SystemTime, UNIX_EPOCH};

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

#[component]
pub(super) fn request_table(
    requests: Signal<Vec<(usize, RequestRow)>>,
    selected: Signal<Option<usize>>,
) {
    view! {
        <DataTable
            rows={requests}
            row_selectable={true}
            cell_selectable={false}
            col_selectable={false}
            loop_selection={true}
            row_header={true}
            on_select_row={|row_ix, cx| selected(Some(row_ix))}
        >
            <Col id="time" title="Time" width={80.} r={|req| row_time(req.1.time)} sortable />
            <Col id="req_type" title="Type" width={80.} r={|req| format!("{:?}", req.1.addr_type)} sortable />
            <Col id="direction" title="Dir" width={80.} r={|req| if req.1.dirn == Dirn::In { "In" } else { "Out" }} sortable />
            <Col id="method" title="Method" width={80.} r={|req| req.1.method.as_str().to_string()} sortable />
            <Col id="url" title="URL" r={|req| req.1.host.clone()} sortable />
            <Col id="status" title="Status" width={80.} r={|req| req.1.status.map(|status| status.as_u16().to_string()).unwrap_or_default()} sortable />
            <Col id="length" title="Length" width={80.} r={|req| req.1.length.to_string()} sortable text_right />
        </DataTable>
    }
}
