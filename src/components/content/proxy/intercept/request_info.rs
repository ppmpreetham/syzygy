use gpui_kit::component::input::{Editor, EditorState};
use gpui_kit::prelude::*;
use gpui_kit::*;
use zopra::{component, view, hooks::use_state};

#[component]
pub fn RequestInfo(content: String) {
    let (get_editor, set_editor) = use_state(None::<gpui_kit::Entity<EditorState>>, window, cx);
    
    if get_editor(cx).is_none() {
        let content_clone = content.clone();
        let ed = cx.new(|cx| {
            EditorState::new(window, cx)
                .line_number(true)
                .folding(true)
                .default_value(content_clone)
        });
        set_editor(Some(ed), cx);
    }
    
    let ed = get_editor(cx).unwrap();
    
    view! {
        <div class="flex flex-col size-full">
            <div class="p-2 border-b border-[#27272a] bg-[#18181b] font-semibold text-sm">
                "Request Info"
            </div>
            <div class="flex-1 size-full relative">
                { Editor::new(&ed).size_full() }
            </div>
        </div>
    }
}

