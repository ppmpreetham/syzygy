use gpui_kit::component::input::{Editor, EditorState};
use gpui_kit::prelude::*;
use gpui_kit::*;
use zopra::{
    component,
    hooks::{Setter, use_state},
    view,
};

#[component]
pub fn RequestInfo(content: SharedString, set_editor_entity: Setter<Option<Entity<EditorState>>>) {
    let (editor, set_editor) = use_state(None::<Entity<EditorState>>);
    let (prev_content, set_prev_content) = use_state(String::new());

    if editor.is_none() {
        let content_clone = content.clone();
        let ed = cx.new(|cx| {
            EditorState::new(window, cx)
                .language("http")
                .line_number(true)
                .folding(true)
                .default_value(content_clone)
        });
        set_editor_entity.set(Some(ed.clone()), cx);
        set_editor(Some(ed));
        set_prev_content(content.clone());
    }

    let Some(ed) = editor.as_ref() else {
        return view! { <div /> };
    };

    if *prev_content != content {
        set_prev_content(content.clone());
        ed.update(cx, |ed, cx| {
            ed.set_value(content.clone(), window, cx);
        });
    }

    view! {
        <div class="flex flex-col size-full">
            <div class="p-2 border-b border-[#27272a] bg-[#18181b] font-semibold text-sm">
                "Request Info"
            </div>
            <div class="flex-1 size-full relative">
                { Editor::new(ed).size_full() }
            </div>
        </div>
    }
}
