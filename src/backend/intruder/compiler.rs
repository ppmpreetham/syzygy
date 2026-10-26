use crate::components::content::intruder::{GRAPHEME_LEN, SectionPosition, sectioner};

pub struct CompiledSegments {
  pub segments: Vec<Vec<u8>>
}

impl CompiledSegments{
  // stores irreplaceable chars in ram once
  pub fn compile(req: &str) -> Self {
    let mut segments: Vec<Vec<u8>> = Vec::new();
    let mut idx = 0usize;
    let sections = sectioner(req).unwrap_or_default();

    for section in sections{
      let st = section.start - GRAPHEME_LEN;
      if st > idx{
        segments.push(req.as_bytes()[idx..st].to_vec());
      }
      idx = section.end + GRAPHEME_LEN;
    }

    if idx < req.len(){
      segments.push(req.as_bytes()[idx..].to_vec());
    }

    Self {segments}
  }

  // reconstruct bad boy back without using memory
  #[inline]
  pub fn reconstruct(&self, payload: &str, buffer: &mut Vec<u8>) {
      buffer.clear();

      if let Some((first, rest)) = self.segments.split_first() {
          buffer.extend_from_slice(first);
          buffer.extend_from_slice(payload.as_bytes());

          for segment in rest {
              buffer.extend_from_slice(segment);
          }
      } else {
          buffer.extend_from_slice(payload.as_bytes());
      }
  }
}
