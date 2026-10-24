use gpui_kit::component::input::EditorState;
use gpui_kit::prelude::*;
use gpui_kit::*;
use zopra::components::Editor;
use zopra::hooks::use_editor;
use zopra::{
    component,
    hooks::{Signal, use_state},
    view,
};

#[component]
pub fn RequestInfo(content: SharedString, editor_entity: Signal<Option<Entity<EditorState>>>) {
    let editor = use_editor();

    let (shown, set_shown) = use_state(None::<SharedString>);
    if shown.read().as_ref() != Some(&content) {
        set_shown(Some(content.clone()));
        editor_entity(Some(editor.clone()));
        editor.update(cx, |editor, cx| editor.set_value(content, window, cx));
    }

    view! {
        <div class="flex flex-col size-full">
            <div class="p-2 border-b border-[#27272a] bg-[#18181b] font-semibold text-sm">
                "Request Info"
            </div>
            <div class="flex-1 size-full relative">
                <Editor
                    state={&editor}
                    language="http"
                    line_numbers={true}
                    folding={true}
                    readonly={true}
                />
            </div>
        </div>
    }
}
