#[derive(Eq, PartialEq, Copy, Clone)]
pub enum CompositionType {
  Direct,
  Abbr,
  Hiragana,
  Katakana,
  EmEisu,
  EnKatakana,
}
