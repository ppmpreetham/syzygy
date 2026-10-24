mod algorithm;
mod editor;
pub mod state;
mod table;
mod types;

use editor::IntruderEditor;
use gpui_kit::base::input;
use gpui_kit::*;
use state::Intruders;
use table::IntruderTable;
use types::AttackType;
use zopra::ui::{Resizable, ResizablePanel, Tab, TabBar};
use zopra::{
    component,
    hooks::{Signal, use_context, use_event, use_input, use_state},
    view,
};

pub(crate) use algorithm::{GRAPHEME_LEN, SectionPosition, sectioner};

#[derive(Clone, PartialEq, Default)]
pub struct VariableRow {
    pub variable: SharedString,
    pub payload: SharedString,
    pub attack_type: AttackType,
}

pub const DUMMY_PAYLOADS: &[&str] = &["Payload 1", "Payload 2", "Payload 3"];


#[component]
pub fn intruder() {
    let intruders: Signal<Intruders> = use_context();
    let input_state = use_input();

    // show the active tab's target when switching tabs
    let (active, request_ids) = {
        let intruders = intruders.read();
        (intruders.active, intruders.tabs.iter().map(|tab| tab.req_id).collect::<Vec<_>>())
    };
    let (shown, set_shown) = use_state(usize::MAX);
    if shown() != active {
        set_shown(active);
        let target = intruders.read().current().map(|tab| tab.target.clone()).unwrap_or_default();
        input_state.update(cx, |input, cx| input.replace_all(target.as_str(), window, cx));
    }

    use_event(&input_state, |event: &input::InputEvent, cx| {
        if matches!(event, input::InputEvent::Change) {
            let target = input_state.read(cx).value();
            intruders(|intruders| {
                if let Some(tab) = intruders.current_mut() {
                    tab.target = target;
                }
            });
        }
    });

    view! {
        <div class="flex flex-col size-full text-[#ededed]">
            <TabBar
                selected_index={active}
                on_click={|index, _, cx| intruders(|intruders| intruders.active = *index)}
                class="px-4"
            >
                {...request_ids.into_iter().enumerate().map(|(i, request_id)| view! {
                    <Tab
                        label={format!("Request {request_id}")}
                        suffix={view! {
                            <div
                                class="text-gray-400 px-1 rounded-sm hover:text-white hover:bg-gray-700 cursor-pointer"
                                on_click={|| intruders(|intruders| intruders.close(i))}
                            >
                                "X "
                            </div>
                        }}
                    />
                })}
            </TabBar>

            <div class="p-2 flex flex-row w-full gap-2 items-center">
              <div>"Target"</div>
              <input state={&input_state} class="py-1 px-2 flex-1" />
            </div>

            <div class="flex-1 size-full">
              <Resizable id="intruder-split" vertical>
                <ResizablePanel size={px(250.)}>
                  <div class="w-full size-full px-10 pt-2 pb-2">
                    <IntruderTable />
                  </div>
                </ResizablePanel>

                <ResizablePanel>
                  <IntruderEditor />
                </ResizablePanel>
              </Resizable>
            </div>
        </div>
    }
}
