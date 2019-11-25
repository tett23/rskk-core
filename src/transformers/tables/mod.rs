mod hiragana;

use super::BufferState;

pub fn hiragana_convert(current_buffer: &str, character: char) -> Option<(String, BufferState)> {
  hiragana::convert(current_buffer, character)
}
