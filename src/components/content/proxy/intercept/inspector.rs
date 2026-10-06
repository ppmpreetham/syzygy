use gpui_kit::*;
use strum::FromRepr;
use zopra::{component, hooks::use_state, view};
#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(usize)]
enum InspectorTab {
    Inspect = 0,
    Notes = 1,
}

#[component]
pub(super) fn inspector(request: String) {
    _ = request;
    let (active_tab, set_active_tab) = use_state(InspectorTab::Inspect);

    view! {
      <div class="flex flex-col size-full bg-[#141517]">
        <nav
          underline
          selected_index={*active_tab as usize}
              on_click={move |index, _, cx| {
                if let Some(tab) = InspectorTab::from_repr(*index) {
              set_active_tab(tab);
            }
          }}
          class="px-4"
        >
          <Tab label="Inspector" />
          <Tab label="Notes" />
        </nav>
        <div class="flex-1 w-full text-[#ededed]">
          {match *active_tab {
            // TODO: match it with other tabs later
            _ => view! { <div></div> }.into_any_element(),
          }}
        </div>
      </div>
    }
}
