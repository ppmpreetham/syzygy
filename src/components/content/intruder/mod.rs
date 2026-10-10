mod algorithm;
mod editor;
mod table;
mod types;
mod state;

use editor::IntruderEditor;
use gpui_kit::base::input;
use gpui_kit::*;
use table::IntruderTable;
use types::AttackType;
use zopra::{component, hooks::use_input, hooks::use_state, view};
use state::IntruderState;

#[derive(Clone, PartialEq, Default)]
pub struct VariableRow {
    pub variable: SharedString,
    pub payload: SharedString,
    pub attack_type: AttackType,
}

pub const DUMMY_PAYLOADS: &[&str] = &["Payload 1", "Payload 2", "Payload 3"];

#[component]
pub fn intruder() {
    let input_state = use_input(window, cx);
    let (get_rows, set_rows) = use_state(|| IntruderState::default());

    view! {
        <div class="flex flex-col size-full text-[#ededed]">
            <div class="p-2 flex flex-row w-full gap-2 items-center">
              <div>"Target"</div>
              <input state={&input_state} class="py-1 px-2 flex-1" />
            </div>

            <div class="flex-1 size-full">
              <Resizable id="intruder-split" vertical>
                <ResizablePanel size={px(250.)}>
                  <div class="w-full size-full px-10 pt-2 pb-2">
                    <IntruderTable rows={(*get_rows(cx)).rows.clone()} set_rows={set_rows.clone()} />
                  </div>
                </ResizablePanel>

                <ResizablePanel>
                  <IntruderEditor set_rows={set_rows.clone()} />
                </ResizablePanel>
              </Resizable>
            </div>
        </div>
    }
}



