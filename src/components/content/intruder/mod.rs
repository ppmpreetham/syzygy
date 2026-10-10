mod types;
mod algorithm;

use gpui_kit::*;
use gpui_kit::base::input;
use zopra::{component, hooks::use_state, view};
use gpui_kit::component::menu::PopupMenuItem;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::menu::DropdownMenu;
use types::AttackType;
use strum::IntoEnumIterator;
use zopra::hooks::use_input;

#[derive(Clone, PartialEq)]
struct VariableRow {
    variable: SharedString,
    payload: SharedString,
    attack_type: AttackType,
}

const DUMMY_PAYLOADS: &[&str] = &[
    "Payload 1",
    "Payload 2",
    "Payload 3",
];

#[component]
pub fn intruder() {
    let input_state = use_input(window, cx);
    let (get_rows, set_rows) = use_state(vec![
        VariableRow {
            variable: "A".into(),
            payload: "Payload 1".into(),
            attack_type: AttackType::Sequential,
        },
        VariableRow {
            variable: "A".into(),
            payload: "Payload 1".into(),
            attack_type: AttackType::Sequential,
        },
        VariableRow {
            variable: "A".into(),
            payload: "Payload 1".into(),
            attack_type: AttackType::Sequential,
        },
    ]);

    let set_rows_payload = set_rows.clone();
    let set_rows_attack = set_rows.clone();

    view! {
        <div class="flex flex-col items-center justify-start size-full text-[#ededed]">
            <div class="p-2 flex flex-row w-full gap-2">
              <div>"Target"</div>
              <input state = {&input_state} class="py-2" />
            </div>

            <div class="w-full h-64 px-10">
              <DataTable rows={get_rows(cx)}>
                  <Col id="var" title="Variable" r={|row| row.variable.clone()} />

                  <Col id="payload" title="Payload" r={|row: &VariableRow| {
                      let target_var = row.variable.clone();
                      let set_rows = set_rows_payload.clone();
                      view! {
                          <button
                              id={format!("payload-{}", row.variable)}
                              label={row.payload.clone()}
                              dropdown_menu={move |mut menu: PopupMenu, _, _| {
                                  for p in DUMMY_PAYLOADS {
                                      let payload_str = p.to_string();
                                      let target_var = target_var.clone();
                                      let set_rows = set_rows.clone();
                                      menu = menu.item(
                                          PopupMenuItem::new(payload_str.clone())
                                              .on_click(move |_, _, cx| {
                                                  set_rows.update(|rows| {
                                                      if let Some(r) = rows.iter_mut().find(|r| r.variable == target_var) {
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
                  }} />

                  <Col id="attack" title="Attack Type" r={|row: &VariableRow| {
                      let target_var = row.variable.clone();
                      let set_rows = set_rows_attack.clone();
                      view! {
                          <button
                              id={format!("attack-{}", row.variable)}
                              label={row.attack_type.to_string()}
                              dropdown_menu={move |mut menu: PopupMenu, _, _| {
                                  for attack in AttackType::iter() {
                                      let attack_clone = attack.clone();
                                      let target_var = target_var.clone();
                                      let set_rows = set_rows.clone();
                                      menu = menu.item(
                                          PopupMenuItem::new(attack.to_string())
                                              .on_click(move |_, _, cx| {
                                                  set_rows.update(|rows| {
                                                      if let Some(r) = rows.iter_mut().find(|r| r.variable == target_var) {
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
                  }} />
              </DataTable>
            </div>

            <div class="p-2 flex flex-row gap-2">
              <div>"Positions"</div>
              <button label="Add §"/>
              <button label="Clear §"/>
              <button label="Auto §"/>
            </div>

            <div class="flex flex-row"></div>
        </div>
    }
}
