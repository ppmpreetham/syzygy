use super::state::Intruders;
use super::types::AttackType;
use super::{DUMMY_PAYLOADS, VariableRow};
use gpui_kit::component::button::Button;
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::*;
use strum::IntoEnumIterator;
use zopra::hooks::{Signal, use_context};
use zopra::{component, view};

#[component]
pub fn IntruderTable() {
    let intruders: Signal<Intruders> = use_context();
    let rows = intruders
        .read()
        .current()
        .map(|tab| tab.rows.clone())
        .unwrap_or_default();

    view! {
        <DataTable rows={rows}>
            <Col id="var" title="Variable" r={|row| view! { <div class="truncate">{ row.variable.clone() }</div> }} />

            <Col id="payload" title="Payload" r={|row: &VariableRow| {
                row_dropdown("payload", row, row.payload.clone(), DUMMY_PAYLOADS.to_vec(), intruders, |row, payload| {
                    row.payload = payload.into();
                })
            }} />

            <Col id="attack" title="Attack Type" r={|row: &VariableRow| {
                row_dropdown("attack", row, row.attack_type.to_string().into(), AttackType::iter().collect(), intruders, |row, attack| {
                    row.attack_type = attack;
                })
            }} />
        </DataTable>
    }
}

fn row_dropdown<T: ToString + Clone + 'static>(
    kind: &str,
    row: &VariableRow,
    label: SharedString,
    options: Vec<T>,
    intruders: Signal<Intruders>,
    apply: fn(&mut VariableRow, T),
) -> impl IntoElement + use<T> {
    let variable = row.variable.clone();
    Button::new(format!("{kind}-{variable}"))
        .label(label)
        .dropdown_menu(move |menu: PopupMenu, _, _| {
            options.iter().fold(menu, |menu, option| {
                let (variable, option) = (variable.clone(), option.clone());
                menu.item(PopupMenuItem::new(option.to_string()).on_click(move |_, _, cx| {
                    intruders.update(
                        |intruders| {
                            if let Some(row) = intruders.row_mut(&variable) {
                                apply(row, option.clone());
                            }
                        },
                        cx,
                    );
                }))
            })
        })
}
