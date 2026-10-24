use super::algorithm::sectioner;
use super::state::{Intruders, add_section, remove_section};
use gpui_kit::HighlightStyle;
use gpui_kit::base::input::InputEvent;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::input::{RangeDecoration, RangeDecorationStyle, TextDecoration};
use gpui_kit::*;
use zopra::components::Editor;
use zopra::hooks::{
    Signal, use_context, use_editor, use_editor_ranges, use_editor_text_styles, use_event,
    use_state,
};
use zopra::{component, view};

#[component]
pub fn IntruderEditor() {
    let intruders: Signal<Intruders> = use_context();
    let editor = use_editor();

    // show the active tab's request when switching tabs
    let active = intruders.read().active;
    let (shown, set_shown) = use_state(usize::MAX);
    if shown() != active {
        set_shown(active);
        let request = intruders.read().current().map(|tab| tab.raw_request.clone()).unwrap_or_default();
        editor.update(cx, |editor, cx| editor.replace_all(&request, window, cx));
    }

    let text = editor.read(cx).value();
    let sections = sectioner(&text).unwrap_or_default();

    // TODO: make these use themes
    use_editor_ranges(
        window,
        cx,
        &editor,
        sections
            .iter()
            .map(|s| {
                RangeDecoration::new(s.start..s.end)
                    .with_style(RangeDecorationStyle::Fill)
                    .with_color(cx.theme().primary.opacity(0.2))
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
                        font_weight: Some(gpui_kit::FontWeight::BOLD),
                        ..Default::default()
                    },
                )
            })
            .collect(),
    );

    use_event(&editor, |event: &InputEvent, cx| {
        if matches!(event, InputEvent::Change) {
            let text = editor.read(cx).value();
            if let Ok(sections) = sectioner(&text) {
                intruders(|intruders| {
                    if let Some(tab) = intruders.current_mut() {
                        tab.set_request(text.to_string(), &sections);
                    }
                });
            }
        }
    });

    view! {
        <div class="flex flex-col size-full">
            <div class="p-2 flex flex-row items-center gap-2 border-t border-[#27272a] bg-[#18181b]">
                <div class="font-semibold text-sm mr-2">"Positions"</div>
                <button label="Add A " on_click={|_, window, cx| add_section(&editor, window, cx)} />
                <button label="Clear A " on_click={|_, window, cx| remove_section(&editor, window, cx)} />
                <button label="Auto A " on_click={|| todo!()} />
            </div>
            <div class="flex-1 size-full relative">
                <Editor state={&editor} language="http" line_numbers={true} />
            </div>
        </div>
    }
}
