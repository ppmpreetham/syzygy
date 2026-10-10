use super::VariableRow;
use super::algorithm::sectioner;
use super::state::{IntruderState, add_section, remove_section};
use gpui_kit::HighlightStyle;
use gpui_kit::base::input::InputEvent;
use gpui_kit::component::input::{RangeDecoration, RangeDecorationStyle, TextDecoration};
use gpui_kit::*;
use zopra::components::Editor;
use zopra::hooks::Setter;
use zopra::hooks::{
    Snap, use_editor, use_editor_ranges, use_editor_text_styles, use_event, use_state,
};
use zopra::{component, view};

#[component]
pub fn IntruderEditor(
    intruders: Snap<Vec<IntruderState>>,
    set_intruders: Setter<Vec<IntruderState>>,
    active_idx: usize,
) {
    let req = intruders.get(active_idx).cloned().unwrap_or_default();
    let editor = use_editor(window, cx);
    let (local_idx, set_local_idx) = use_state(usize::MAX);
    if *local_idx != active_idx {
        set_local_idx.set(active_idx, cx);
        let text_to_set = req.raw_request.clone();
        editor.update(cx, |ed, cx| {
            ed.replace_all(text_to_set.as_str(), window, cx);
        });
    }
    let sel = editor.read(cx).selected_range();
    let (start, end) = (sel.start, sel.end);
    let text = editor.read(cx).value();
    let selected_text = text[sel].to_string();
    let sections = sectioner(&text).unwrap_or_default();

    use_editor_ranges(
        window,
        cx,
        &editor,
        sections
            .iter()
            .map(|s| {
                RangeDecoration::new(s.start..s.end)
                    .with_style(RangeDecorationStyle::Fill)
                    .with_color(gpui_kit::rgba(0xFF000033).into())
            })
            .collect(),
    );

    use_editor_text_styles(
        window,
        cx,
        &editor,
        sections
            .iter()
            .map(|s| {
                TextDecoration::new(
                    s.start..s.end,
                    HighlightStyle {
                        color: Some(gpui_kit::rgba(0x00FF00FF).into()),
                        font_weight: Some(gpui_kit::FontWeight::BOLD),
                        ..Default::default()
                    },
                )
            })
            .collect(),
    );

    let editor_clone = editor.clone();
    use_event(&editor, move |event: &InputEvent, cx| {
        if matches!(event, InputEvent::Change) {
            let text = editor_clone.read(cx).value();
            if let Ok(sections) = sectioner(&text) {
                set_intruders.update(
                    |list| {
                        if let Some(state) = list.get_mut(active_idx) {
                            state.raw_request = text.to_string();
                            let mut new_rows = Vec::new();
                            for s in &sections {
                                if let Some(existing) =
                                    state.rows.iter().find(|r| r.variable == s.word)
                                {
                                    new_rows.push(existing.clone());
                                } else {
                                    new_rows.push(VariableRow {
                                        variable: s.word.into(),
                                        ..Default::default()
                                    });
                                }
                            }
                            state.rows = new_rows;
                        }
                    },
                    cx,
                );
            }
        }
    });

    view! {
        <div class="flex flex-col size-full">
            <div class="p-2 flex flex-row items-center gap-2 border-t border-[#27272a] bg-[#18181b]">
                <div class="font-semibold text-sm mr-2">"Positions"</div>
                <button label="Add A " on_click={ { let editor = editor.clone(); move |_, window, cx| add_section(&editor, window, cx) } } />
                <button label="Clear A " on_click={ { let editor = editor.clone(); move |_, window, cx| remove_section(&editor, window, cx) } } />
                <button label="Auto A " on_click={move |_, _, _| todo!()} />
            </div>
            <div class="flex-1 size-full relative">
                <Editor state={&editor} language="http" line_numbers={true} />
            </div>
        </div>
    }
}
