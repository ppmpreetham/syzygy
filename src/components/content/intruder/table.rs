use super::{VariableRow, DUMMY_PAYLOADS};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::*;
use zopra::{component, hooks::Setter, view};
use super::types::AttackType;
use strum::IntoEnumIterator;

#[component]
pub fn IntruderTable(rows: Vec<VariableRow>, set_rows: Setter<Vec<VariableRow>>) {
    view! {
        <DataTable rows={rows}>
            <Col id="var" title="Variable" r={|row| row.variable.clone()} />
            
            <Col id="payload" title="Payload" r={|row: &VariableRow| {
                render_payload_dropdown(row.id, row.variable.clone(), row.payload.clone(), set_rows.clone())
            }} />
            
            <Col id="attack" title="Attack Type" r={|row: &VariableRow| {
                render_attack_dropdown(row.id, row.variable.clone(), row.attack_type.clone(), set_rows.clone())
            }} />
        </DataTable>
    }
}

fn render_payload_dropdown(target_id: usize, variable: SharedString, payload: SharedString, set_rows: Setter<Vec<VariableRow>>) -> impl IntoElement {
    view! {
        <button
            id={format!("payload-{}", variable)}
            label={payload}
            dropdown_menu={move |mut menu: PopupMenu, _, _| {
                for p in DUMMY_PAYLOADS {
                    let payload_str = p.to_string();
                    let set_rows = set_rows.clone();
                    menu = menu.item(
                        PopupMenuItem::new(payload_str.clone())
                            .on_click(move |_, _, cx| {
                                set_rows.update(|rows| {
                                    if let Some(r) = rows.iter_mut().find(|r| r.id == target_id) {
                                        r.payload = payload_str.clone().into();
                                    }
                                }, cx);
                            })
                    );
                }
                menu
            }}
        />
    }
}

fn render_attack_dropdown(target_id: usize, variable: SharedString, attack_type: AttackType, set_rows: Setter<Vec<VariableRow>>) -> impl IntoElement {
    view! {
        <button
            id={format!("attack-{}", variable)}
            label={attack_type.to_string()}
            dropdown_menu={move |mut menu: PopupMenu, _, _| {
                for attack in AttackType::iter() {
                    let attack_clone = attack.clone();
                    let set_rows = set_rows.clone();
                    menu = menu.item(
                        PopupMenuItem::new(attack.to_string())
                            .on_click(move |_, _, cx| {
                                set_rows.update(|rows| {
                                    if let Some(r) = rows.iter_mut().find(|r| r.id == target_id) {
                                        r.attack_type = attack_clone.clone();
                                    }
                                }, cx);
                            })
                    );
                }
                menu
            }}
        />
    }
}
