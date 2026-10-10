pub struct SectionPosition<'a> {
  /// starting char
  pub start: usize,
  /// ending char
  pub end: usize,
  /// the word
  pub word: &'a str,
}

const GRAPHEME_LEN: usize = '§'.len_utf8();

// fucking stores each placeholder, just like burpsuite baddies
pub fn sectioner(text: &str) -> Result<Vec<SectionPosition<'_>>, usize> {
  let mut sections = Vec::new();
  let mut pos = 0;
  let mut last_i = 0;

  text.split("§").enumerate().for_each(|(i, word)| {
    last_i = i;
    if (i & 1) == 1 {
      sections.push(SectionPosition {
        start: pos,
        end: pos + word.len(),
        word
      });
    }
    pos += word.len() + GRAPHEME_LEN;
  });

  if (last_i & 1) == 1 {
    let unclosed_pos = sections.last().map_or(0, |s| s.start - GRAPHEME_LEN);
    return Err(unclosed_pos);
  }
  Ok(sections)
}
