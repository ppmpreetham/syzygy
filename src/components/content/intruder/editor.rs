use gpui_kit::component::input::{RangeDecoration, RangeDecorationStyle, TextDecoration};
use gpui_kit::HighlightStyle;
use gpui_kit::*;
use zopra::components::Editor;
use zopra::hooks::{use_editor, use_editor_ranges, use_editor_text_styles, use_event};
use gpui_kit::base::input::InputEvent;
use super::VariableRow;
use zopra::{component, view};
use super::state::{IntruderState, add_section, remove_section};
use super::algorithm::sectioner;
use zopra::hooks::Setter;

#[component]
pub fn IntruderEditor(set_rows: Setter<IntruderState>) {
    let editor = use_editor(window, cx);
    let sel = editor.read(cx).selected_range();
    let (start, end) = (sel.start, sel.end);
    let text = editor.read(cx).value();
    let selected_text = text[sel].to_string();
    let sections = sectioner(&text).unwrap_or_default();

    use_editor_ranges(
        window,
        cx,
        &editor,
        sections.iter().map(|s| {
            RangeDecoration::new(s.start..s.end)
                .with_style(RangeDecorationStyle::Fill)
                .with_color(gpui_kit::rgba(0xFF000033).into())
        }).collect(),
    );

    use_editor_text_styles(
        window,
        cx,
        &editor,
        sections.iter().map(|s| {
            TextDecoration::new(s.start..s.end, HighlightStyle {
                color: Some(gpui_kit::rgba(0x00FF00FF).into()),
                font_weight: Some(gpui_kit::FontWeight::BOLD),
                ..Default::default()
            })
        }).collect(),
    );


    let editor_clone = editor.clone();
    use_event(&editor, move |event: &InputEvent, cx| {
        if matches!(event, InputEvent::Change) {
            let text = editor_clone.read(cx).value();
            if let Ok(sections) = sectioner(&text) {
                set_rows.update(|state| {
                    let mut new_rows = Vec::new();
                    for s in &sections {
                        if let Some(existing) = state.rows.iter().find(|r| r.variable == s.word) {
                            new_rows.push(existing.clone());
                        } else {
                            new_rows.push(VariableRow { variable: s.word.into(), ..Default::default() });
                        }
                    }
                    state.rows = new_rows;
                }, cx);
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
