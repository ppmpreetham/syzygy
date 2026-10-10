use super::{VariableRow, DUMMY_PAYLOADS};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::component::button::Button;
use gpui_kit::*;
use zopra::{component, hooks::Setter, view};
use super::types::AttackType;
use strum::IntoEnumIterator;
use super::state::IntruderState;

#[component]
pub fn IntruderTable(rows: Vec<VariableRow>, set_rows: Setter<IntruderState>) {
    view! {
        <DataTable rows={rows}>
            <Col id="var" title="Variable" r={|row| row.variable.clone()} />

            <Col id="payload" title="Payload" r={|row: &VariableRow| {
                render_payload_dropdown(row.variable.clone(), row.payload.clone(), set_rows.clone())
            }} />

            <Col id="attack" title="Attack Type" r={|row: &VariableRow| {
                render_attack_dropdown(row.variable.clone(), row.attack_type.clone(), set_rows.clone())
            }} />
        </DataTable>
    }
}

fn render_payload_dropdown(variable: SharedString, payload: SharedString, set_rows: Setter<IntruderState>) -> impl IntoElement {
    let target_var = variable.clone();

    Button::new(format!("payload-{variable}"))
        .label(payload)
        .dropdown_menu(move |mut menu: PopupMenu, _, _| {
            for p in DUMMY_PAYLOADS {
                let payload_str = p.to_string();
                let set_rows = set_rows.clone();
                let target_var = target_var.clone();
                menu = menu.item(
                    PopupMenuItem::new(payload_str.clone())
                        .on_click(move |_, _, cx| {
                            set_rows.update(|state| {
                                if let Some(r) = state.rows.iter_mut().find(|r| r.variable == target_var) {
                                    r.payload = payload_str.clone().into();
                                }
                            }, cx);
                        })
                );
            }
            menu
        })
}

fn render_attack_dropdown(variable: SharedString, attack_type: AttackType, set_rows: Setter<IntruderState>) -> impl IntoElement {
    let target_var = variable.clone();

    Button::new(format!("attack-{}", variable))
        .label(attack_type.to_string())
        .dropdown_menu(move |mut menu: PopupMenu, _, _| {
            for attack in AttackType::iter() {
                let attack_clone = attack.clone();
                let set_rows = set_rows.clone();
                let target_var = target_var.clone();
                menu = menu.item(
                    PopupMenuItem::new(attack.to_string())
                        .on_click(move |_, _, cx| {
                            set_rows.update(|state| {
                                if let Some(r) = state.rows.iter_mut().find(|r| r.variable == target_var) {
                                    r.attack_type = attack_clone.clone();
                                }
                            }, cx);
                        })
                );
            }
            menu
        })
}
