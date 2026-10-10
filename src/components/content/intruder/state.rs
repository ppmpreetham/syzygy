use gpui_kit::{Entity, Window, App, base::input::EditorState};
use wry::http::Request;
use super::algorithm::{sectioner, SectionPosition, GRAPHEME_LEN};
use super::VariableRow;
use gpui_kit::SharedString;

#[derive(Clone, PartialEq)]
pub struct IntruderState {
    pub req_id: usize,
    pub target: SharedString,
    pub raw_request: String,
    pub update_host_header: bool,
    pub rows: Vec<VariableRow>,
}
impl Default for IntruderState {
    fn default() -> Self {
        Self {
            req_id: 1,
            target: "".into(),
            raw_request: String::new(),
            update_host_header: true,
            rows: Vec::new(),
        }
    }
}

impl IntruderState {
    pub fn new(req_id: usize, target: SharedString, raw_request: String) -> Self {
        Self {
            req_id,
            target,
            raw_request,
            update_host_header: true,
            rows: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, word: &str) {
        if !word.is_empty() {
          let row = VariableRow {variable: word.into(), ..Default::default()};
          self.rows.push(row);
        }
    }

    pub fn remove_variable(&mut self, word: &str) {
      self.rows.retain(|row| row.variable != word);
    }

    pub fn auto_variables(&mut self) {
        todo!()
    }
}

pub fn add_section(
    editor: &Entity<EditorState>,
    window: &mut Window,
    cx: &mut App,
) {
    editor.update(cx, |state, cx| {
        let sel = state.selected_range();
        if sel.start == sel.end {
            return; // nothing selected, nothing to wrap
        }
        let text = state.value();
        let selected = text[sel.start..sel.end].to_string();
        state.replace(format!("§{selected}§"), window, cx);
    });
}

pub fn remove_section(
    editor: &Entity<EditorState>,
    window: &mut Window,
    cx: &mut App,
) {
    editor.update(cx, |state, cx| {
        let text = state.value();
        let sel = state.selected_range();
        let Ok(sections) = sectioner(&text) else {
            return;
        };

        let target = sections.iter().find(|s| {
            let block_start = s.start.saturating_sub(GRAPHEME_LEN);
            let block_end = s.end + GRAPHEME_LEN;
            if sel.start == sel.end {
                block_start <= sel.start && sel.start <= block_end
            } else {
                sel.start < block_end && sel.end > block_start
            }
        });

        if let Some(section) = target {
            let full_range = (section.start - GRAPHEME_LEN)..(section.end + GRAPHEME_LEN);
            state.set_selected_range(full_range, cx);
            state.replace(section.word.to_string(), window, cx);
        }
    });
}
