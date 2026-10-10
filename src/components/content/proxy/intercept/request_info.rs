use gpui_kit::component::input::EditorState;
use gpui_kit::prelude::*;
use gpui_kit::*;
use zopra::{
    component,
    hooks::{Setter, use_state},
    view,
};
use zopra::hooks::use_editor;
use zopra::components::Editor;

#[component]
pub fn RequestInfo(content: SharedString, set_editor_entity: Setter<Option<Entity<EditorState>>>) {
    let editor = use_editor(window, cx);
    let (prev_content, set_prev_content) = use_state(String::new());
    let (is_init, set_is_init) = use_state(false);

    if !*is_init {
        set_is_init(true);
        set_editor_entity.set(Some(editor.clone()), cx);
        set_prev_content(content.clone());
        let content_clone = content.clone();
        editor.update(cx, |ed, cx| {
            ed.set_value(content_clone, window, cx);
        });
    }

    if *prev_content != content {
        set_prev_content(content.clone());
        editor.update(cx, |ed, cx| {
            ed.set_value(content.clone(), window, cx);
        });
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
