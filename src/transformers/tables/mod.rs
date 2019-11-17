mod hiragana;

use super::BufferState;

pub fn hiragana_convert(current_buffer: &str, character: char) -> Option<(String, BufferState)> {
  return hiragana::convert(current_buffer, character);
}
