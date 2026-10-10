use gpui_kit::component::input::{RangeDecoration, RangeDecorationStyle, TextDecoration};
use gpui_kit::HighlightStyle;
use gpui_kit::*;
use zopra::components::Editor;
use zopra::hooks::{use_editor, use_editor_ranges, use_editor_text_styles};
use zopra::{component, view};

#[component]
pub fn IntruderEditor() {
    let editor = use_editor(window, cx);

    use_editor_ranges(
        window,
        cx,
        &editor,
        vec![RangeDecoration::new(10..20)
            .with_style(RangeDecorationStyle::Fill)
            .with_color(gpui_kit::rgba(0xFF000033).into())],
    );

    use_editor_text_styles(
        window,
        cx,
        &editor,
        vec![TextDecoration::new(
            30..40,
            HighlightStyle {
                color: Some(gpui_kit::rgba(0x00FF00FF).into()),
                font_weight: Some(gpui_kit::FontWeight::BOLD),
                ..Default::default()
            },
        )],
    );

    view! {
        <div class="flex flex-col size-full">
            <div class="p-2 flex flex-row items-center gap-2 border-t border-[#27272a] bg-[#18181b]">
                <div class="font-semibold text-sm mr-2">"Positions"</div>
                <button label="Add A "/>
                <button label="Clear A "/>
                <button label="Auto A "/>
            </div>
            <div class="flex-1 size-full relative">
                <Editor state={&editor} language="http" line_numbers={true} />
            </div>
        </div>
    }
}
