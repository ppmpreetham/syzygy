mod algorithm;
mod editor;
mod table;
mod types;
pub mod state;

use editor::IntruderEditor;
use gpui_kit::base::input;
use gpui_kit::*;
use table::IntruderTable;
use types::AttackType;
use zopra::{component, hooks::{use_event, use_input, use_state}, view};
use state::IntruderState;
use zopra::hooks::{Snap, Setter};

#[derive(Clone, PartialEq, Default)]
pub struct VariableRow {
    pub variable: SharedString,
    pub payload: SharedString,
    pub attack_type: AttackType,
}

pub const DUMMY_PAYLOADS: &[&str] = &["Payload 1", "Payload 2", "Payload 3"];

#[component]
pub fn intruder(intruders: Snap<Vec<IntruderState>>, set_intruders: Setter<Vec<IntruderState>>, intruder_tab: Snap<usize>, set_intruder_tab: Setter<usize>) {
    let input_state = use_input(window, cx);



    let active_idx = if *intruder_tab >= intruders.len() { intruders.len().saturating_sub(1) } else { *intruder_tab };
    let active_req = intruders.get(active_idx).cloned().unwrap_or_default();

    let (local_idx, set_local_idx) = use_state(usize::MAX);
    if *local_idx != active_idx {
        set_local_idx.set(active_idx, cx);
        let target = active_req.target.clone();
        input_state.update(cx, |inp, cx| { inp.replace_all(target.as_str(), window, cx); });
    }

    let input_evt = input_state.clone();
    let set_intruders_evt = set_intruders.clone();
    use_event(&input_state, move |event: &input::InputEvent, cx| {
        if matches!(event, input::InputEvent::Change) {
            let text = input_evt.read(cx).value();
            set_intruders_evt.update(|list| {
                if let Some(req) = list.get_mut(active_idx) {
                    req.target = text;
                }
            }, cx);
        }
    });

    view! {
        <div class="flex flex-col size-full text-[#ededed]">
            <nav
                selected_index={active_idx}
                on_click={move |index, _, cx| {
                    set_intruder_tab.set(*index, cx);
                }}
                class="px-4"
            >
                {...intruders.iter().enumerate().map(|(i, req)| {
                    let set_intruders = set_intruders.clone();
                    let set_intruder_tab = set_intruder_tab.clone();
                    view! {
                        <Tab
                            label={format!("Request {}", req.req_id)}
                            suffix={view! {
                                <div
                                    class="text-gray-400 px-1 rounded-sm hover:text-white hover:bg-gray-700 cursor-pointer"
                                    on_click={move |_, _, cx| {
                                        set_intruders.update(|list| {
                                            if list.len() > 1 {
                                                list.remove(i);
                                            }
                                        }, cx);
                                        set_intruder_tab.set(0usize, cx);
                                    }}
                                >
                                    "X "
                                </div>
                            }}
                        />
                    }
                })}
            </nav>

            <div class="p-2 flex flex-row w-full gap-2 items-center">
              <div>"Target"</div>
              <input state={&input_state} class="py-1 px-2 flex-1" />
            </div>

            <div class="flex-1 size-full">
              <Resizable id="intruder-split" vertical>
                <ResizablePanel size={px(250.)}>
                  <div class="w-full size-full px-10 pt-2 pb-2">
                    <IntruderTable intruders={intruders.clone()} set_intruders={set_intruders.clone()} active_idx={active_idx} />
                  </div>
                </ResizablePanel>

                <ResizablePanel>
                  <IntruderEditor intruders={intruders.clone()} set_intruders={set_intruders.clone()} active_idx={active_idx} />
                </ResizablePanel>
              </Resizable>
            </div>
        </div>
    }
}
