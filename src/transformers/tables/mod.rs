mod buffer_pair;
mod buffer_pairs;
mod hiragana;

use super::BufferState;

pub use buffer_pair::BufferPair;
pub use buffer_pairs::BufferPairs;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum LetterType {
  Direct,
  Hiragana,
  Katakana,
  EnKatakana,
  EmEisu,
}
