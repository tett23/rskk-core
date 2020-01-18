use super::super::BufferState::*;
use super::{BufferPair, LetterType};
use LetterType::*;

pub fn convert(current_buffer: &str, character: char) -> Option<Vec<BufferPair>> {
  let character = character.to_lowercase().next()?;

  let pairs = match (current_buffer, character) {
    ("", 'a') => Some(vec![("あ", Stop)]),
    ("", 'i') => Some(vec![("い", Stop)]),
    ("", 'u') => Some(vec![("う", Stop)]),
    ("", 'e') => Some(vec![("え", Stop)]),
    ("", 'o') => Some(vec![("お", Stop)]),

    ("", 'k') => Some(vec![("k", Continue)]),
    ("k", 'a') => Some(vec![("か", Stop)]),
    ("k", 'i') => Some(vec![("き", Stop)]),
    ("k", 'u') => Some(vec![("く", Stop)]),
    ("k", 'e') => Some(vec![("け", Stop)]),
    ("k", 'o') => Some(vec![("こ", Stop)]),

    ("", 'g') => Some(vec![("g", Continue)]),
    ("g", 'a') => Some(vec![("が", Stop)]),
    ("g", 'i') => Some(vec![("ぎ", Stop)]),
    ("g", 'u') => Some(vec![("ぐ", Stop)]),
    ("g", 'e') => Some(vec![("げ", Stop)]),
    ("g", 'o') => Some(vec![("ご", Stop)]),

    ("", 's') => Some(vec![("s", Continue)]),
    ("s", 'a') => Some(vec![("さ", Stop)]),
    ("s", 'i') => Some(vec![("し", Stop)]),
    ("s", 'u') => Some(vec![("す", Stop)]),
    ("s", 'e') => Some(vec![("せ", Stop)]),
    ("s", 'o') => Some(vec![("そ", Stop)]),

    ("", 'z') => Some(vec![("z", Continue)]),
    ("z", 'a') => Some(vec![("ざ", Stop)]),
    ("z", 'i') => Some(vec![("じ", Stop)]),
    ("z", 'u') => Some(vec![("ず", Stop)]),
    ("z", 'e') => Some(vec![("で", Stop)]),
    ("z", 'o') => Some(vec![("ど", Stop)]),

    ("", 't') => Some(vec![("t", Continue)]),
    ("t", 'a') => Some(vec![("た", Stop)]),
    ("t", 'i') => Some(vec![("ち", Stop)]),
    ("t", 'u') => Some(vec![("つ", Stop)]),
    ("t", 'e') => Some(vec![("て", Stop)]),
    ("t", 'o') => Some(vec![("と", Stop)]),

    ("t", 's') => Some(vec![("ts", Continue)]),
    ("ts", 'a') => Some(vec![("つ", Stop), ("ぁ", Stop)]),
    ("ts", 'i') => Some(vec![("つ", Stop), ("ぃ", Stop)]),
    ("ts", 'u') => Some(vec![("つ", Stop)]),
    ("ts", 'e') => Some(vec![("つ", Stop), ("ぇ", Stop)]),
    ("ts", 'o') => Some(vec![("つ", Stop), ("ぉ", Stop)]),

    ("", 'd') => Some(vec![("d", Continue)]),
    ("d", 'a') => Some(vec![("だ", Stop)]),
    ("d", 'i') => Some(vec![("ぢ", Stop)]),
    ("d", 'u') => Some(vec![("づ", Stop)]),
    ("d", 'e') => Some(vec![("で", Stop)]),
    ("d", 'o') => Some(vec![("ど", Stop)]),
    ("d", 'h') => Some(vec![("dh", Continue)]),
    ("dh", 'a') => Some(vec![("で", Stop), ("ゃ", Stop)]),
    ("dh", 'i') => Some(vec![("で", Stop), ("ぃ", Stop)]),
    ("dh", 'u') => Some(vec![("で", Stop), ("ゅ", Stop)]),
    ("dh", 'e') => Some(vec![("で", Stop), ("ぇ", Stop)]),
    ("dh", 'o') => Some(vec![("で", Stop), ("ょ", Stop)]),
    ("d", 'y') => Some(vec![("dy", Continue)]),
    ("dy", 'a') => Some(vec![("ぢ", Stop), ("ゃ", Stop)]),
    ("dy", 'i') => Some(vec![("ぢ", Stop), ("ぃ", Stop)]),
    ("dy", 'u') => Some(vec![("ぢ", Stop), ("ゅ", Stop)]),
    ("dy", 'e') => Some(vec![("ぢ", Stop), ("ぇ", Stop)]),
    ("dy", 'o') => Some(vec![("ぢ", Stop), ("ょ", Stop)]),

    ("", 'c') => Some(vec![("c", Continue)]),
    ("c", 'h') => Some(vec![("ch", Continue)]),
    ("ch", 'a') => Some(vec![("ち", Stop), ("ゃ", Stop)]),
    ("ch", 'i') => Some(vec![("ち", Stop)]),
    ("ch", 'u') => Some(vec![("ち", Stop), ("ゅ", Stop)]),
    ("ch", 'e') => Some(vec![("ち", Stop), ("ぇ", Stop)]),
    ("ch", 'o') => Some(vec![("ち", Stop), ("ょ", Stop)]),

    ("c", 'y') => Some(vec![("cy", Continue)]),
    ("cy", 'a') => Some(vec![("ち", Stop), ("ゃ", Stop)]),
    ("cy", 'i') => Some(vec![("ち", Stop), ("ぃ", Stop)]),
    ("cy", 'u') => Some(vec![("ち", Stop), ("ゅ", Stop)]),
    ("cy", 'e') => Some(vec![("ち", Stop), ("ぇ", Stop)]),
    ("cy", 'o') => Some(vec![("ち", Stop), ("ょ", Stop)]),

    ("", 'n') => Some(vec![("n", Continue)]),
    ("n", 'a') => Some(vec![("な", Stop)]),
    ("n", 'i') => Some(vec![("に", Stop)]),
    ("n", 'u') => Some(vec![("ぬ", Stop)]),
    ("n", 'e') => Some(vec![("ね", Stop)]),
    ("n", 'o') => Some(vec![("の", Stop)]),
    ("n", 'n') => Some(vec![("ん", Stop)]),

    ("", 'h') => Some(vec![("h", Continue)]),
    ("h", 'a') => Some(vec![("は", Stop)]),
    ("h", 'i') => Some(vec![("ひ", Stop)]),
    ("h", 'u') => Some(vec![("ふ", Stop)]),
    ("h", 'e') => Some(vec![("へ", Stop)]),
    ("h", 'o') => Some(vec![("ほ", Stop)]),

    ("", 'p') => Some(vec![("p", Continue)]),
    ("p", 'a') => Some(vec![("ぱ", Stop)]),
    ("p", 'i') => Some(vec![("ぴ", Stop)]),
    ("p", 'u') => Some(vec![("ぷ", Stop)]),
    ("p", 'e') => Some(vec![("ぺ", Stop)]),
    ("p", 'o') => Some(vec![("ぽ", Stop)]),

    ("", 'b') => Some(vec![("b", Continue)]),
    ("b", 'a') => Some(vec![("ば", Stop)]),
    ("b", 'i') => Some(vec![("び", Stop)]),
    ("b", 'u') => Some(vec![("ぶ", Stop)]),
    ("b", 'e') => Some(vec![("べ", Stop)]),
    ("b", 'o') => Some(vec![("ぼ", Stop)]),

    ("", 'f') => Some(vec![("f", Continue)]),
    ("f", 'a') => Some(vec![("ふ", Stop), ("ぁ", Stop)]),
    ("f", 'i') => Some(vec![("ふ", Stop), ("ぃ", Stop)]),
    ("f", 'u') => Some(vec![("ふ", Stop)]),
    ("f", 'e') => Some(vec![("ふ", Stop), ("ぇ", Stop)]),
    ("f", 'o') => Some(vec![("ふ", Stop), ("ぉ", Stop)]),

    ("", 'm') => Some(vec![("m", Continue)]),
    ("m", 'a') => Some(vec![("ま", Stop)]),
    ("m", 'i') => Some(vec![("み", Stop)]),
    ("m", 'u') => Some(vec![("む", Stop)]),
    ("m", 'e') => Some(vec![("め", Stop)]),
    ("m", 'o') => Some(vec![("も", Stop)]),

    ("", 'y') => Some(vec![("y", Continue)]),
    ("y", 'a') => Some(vec![("や", Stop)]),
    ("y", 'i') => Some(vec![("い", Stop)]),
    ("y", 'u') => Some(vec![("ゆ", Stop)]),
    ("y", 'e') => Some(vec![("い", Stop), ("ぇ", Stop)]),
    ("y", 'o') => Some(vec![("よ", Stop)]),

    ("", 'j') => Some(vec![("j", Continue)]),
    ("j", 'a') => Some(vec![("じ", Stop), ("ゃ", Stop)]),
    ("j", 'i') => Some(vec![("じ", Stop)]),
    ("j", 'u') => Some(vec![("じ", Stop), ("ゅ", Stop)]),
    ("j", 'e') => Some(vec![("じ", Stop), ("ぇ", Stop)]),
    ("j", 'o') => Some(vec![("じ", Stop), ("ょ", Stop)]),

    ("j", 'y') => Some(vec![("jy", Continue)]),
    ("jy", 'a') => Some(vec![("じ", Stop), ("ゃ", Stop)]),
    ("jy", 'i') => Some(vec![("じ", Stop), ("ぃ", Stop)]),
    ("jy", 'u') => Some(vec![("じ", Stop), ("ゅ", Stop)]),
    ("jy", 'e') => Some(vec![("じ", Stop), ("ぇ", Stop)]),
    ("jy", 'o') => Some(vec![("じ", Stop), ("ょ", Stop)]),

    ("", 'r') => Some(vec![("r", Continue)]),
    ("r", 'a') => Some(vec![("ら", Stop)]),
    ("r", 'i') => Some(vec![("り", Stop)]),
    ("r", 'u') => Some(vec![("る", Stop)]),
    ("r", 'e') => Some(vec![("れ", Stop)]),
    ("r", 'o') => Some(vec![("ろ", Stop)]),

    ("", 'w') => Some(vec![("w", Continue)]),
    ("w", 'a') => Some(vec![("わ", Stop)]),
    ("w", 'i') => Some(vec![("う", Stop), ("ぃ", Stop)]),
    ("w", 'u') => Some(vec![("う", Stop)]),
    ("w", 'e') => Some(vec![("う", Stop), ("ぇ", Stop)]),
    ("w", 'o') => Some(vec![("を", Stop)]),

    ("", 'v') => Some(vec![("v", Continue)]),
    ("v", 'a') => Some(vec![("う", Stop), ("゛", Stop), ("ぁ", Stop)]),
    ("v", 'i') => Some(vec![("う", Stop), ("゛", Stop), ("ぃ", Stop)]),
    ("v", 'u') => Some(vec![("う", Stop), ("゛", Stop)]),
    ("v", 'e') => Some(vec![("う", Stop), ("゛", Stop), ("ぇ", Stop)]),
    ("v", 'o') => Some(vec![("ぼ", Stop)]),

    ("", 'x') => Some(vec![("x", Continue)]),
    ("x", 'a') => Some(vec![("ぁ", Stop)]),
    ("x", 'i') => Some(vec![("ぃ", Stop)]),
    ("x", 'u') => Some(vec![("ぅ", Stop)]),
    ("x", 'e') => Some(vec![("ぇ", Stop)]),
    ("x", 'o') => Some(vec![("ぉ", Stop)]),

    ("x", 'y') => Some(vec![("xy", Continue)]),
    ("xy", 'a') => Some(vec![("ゃ", Stop)]),
    ("xy", 'i') => Some(vec![("い", Stop)]),
    ("xy", 'u') => Some(vec![("ゅ", Stop)]),
    ("xy", 'e') => Some(vec![("え", Stop)]),
    ("xy", 'o') => Some(vec![("ぉ", Stop)]),

    // 「tte」 -> 「って」のような促音のルール
    // TODO: 「kkkk」 -> 「っっっk」のような連続した促音のルールが未実装
    ("w", 'w') => Some(vec![("っ", Stop), ("w", Continue)]),
    ("r", 'r') => Some(vec![("っ", Stop), ("r", Continue)]),
    ("t", 't') => Some(vec![("っ", Stop), ("t", Continue)]),
    ("y", 'y') => Some(vec![("っ", Stop), ("y", Continue)]),
    ("p", 'p') => Some(vec![("っ", Stop), ("p", Continue)]),
    ("s", 's') => Some(vec![("っ", Stop), ("s", Continue)]),
    ("d", 'd') => Some(vec![("っ", Stop), ("d", Continue)]),
    ("g", 'g') => Some(vec![("っ", Stop), ("g", Continue)]),
    ("h", 'h') => Some(vec![("っ", Stop), ("h", Continue)]),
    ("j", 'j') => Some(vec![("っ", Stop), ("j", Continue)]),
    ("k", 'k') => Some(vec![("っ", Stop), ("k", Continue)]),
    ("l", 'l') => Some(vec![("っ", Stop), ("l", Continue)]),
    ("z", 'z') => Some(vec![("っ", Stop), ("z", Continue)]),
    ("x", 'x') => Some(vec![("っ", Stop), ("x", Continue)]),
    ("c", 'c') => Some(vec![("っ", Stop), ("c", Continue)]),
    ("v", 'v') => Some(vec![("っ", Stop), ("v", Continue)]),
    ("b", 'b') => Some(vec![("っ", Stop), ("b", Continue)]),
    ("m", 'm') => Some(vec![("っ", Stop), ("m", Continue)]),

    // 「kanji」を「かんじ」にするように、「n」での「ん」の入力ルール
    ("n", 'w') => Some(vec![("ん", Stop), ("w", Continue)]),
    ("n", 'r') => Some(vec![("ん", Stop), ("r", Continue)]),
    ("n", 't') => Some(vec![("ん", Stop), ("t", Continue)]),
    ("n", 'y') => Some(vec![("ん", Stop), ("y", Continue)]),
    ("n", 'p') => Some(vec![("ん", Stop), ("p", Continue)]),
    ("n", 's') => Some(vec![("ん", Stop), ("s", Continue)]),
    ("n", 'd') => Some(vec![("ん", Stop), ("d", Continue)]),
    ("n", 'g') => Some(vec![("ん", Stop), ("g", Continue)]),
    ("n", 'h') => Some(vec![("ん", Stop), ("h", Continue)]),
    ("n", 'j') => Some(vec![("ん", Stop), ("j", Continue)]),
    ("n", 'k') => Some(vec![("ん", Stop), ("k", Continue)]),
    ("n", 'l') => Some(vec![("ん", Stop), ("l", Continue)]),
    ("n", 'z') => Some(vec![("ん", Stop), ("z", Continue)]),
    ("n", 'x') => Some(vec![("ん", Stop), ("x", Continue)]),
    ("n", 'c') => Some(vec![("ん", Stop), ("c", Continue)]),
    ("n", 'v') => Some(vec![("ん", Stop), ("v", Continue)]),
    ("n", 'b') => Some(vec![("ん", Stop), ("b", Continue)]),
    ("n", 'm') => Some(vec![("ん", Stop), ("m", Continue)]),

    // 数字類
    ("", '1') => Some(vec![("1", Stop)]),
    ("", '2') => Some(vec![("2", Stop)]),
    ("", '3') => Some(vec![("3", Stop)]),
    ("", '4') => Some(vec![("4", Stop)]),
    ("", '5') => Some(vec![("5", Stop)]),
    ("", '6') => Some(vec![("6", Stop)]),
    ("", '7') => Some(vec![("7", Stop)]),
    ("", '8') => Some(vec![("8", Stop)]),
    ("", '9') => Some(vec![("9", Stop)]),
    ("", '0') => Some(vec![("0", Stop)]),

    // 記号類
    ("", ',') => Some(vec![("、", Stop)]),
    ("", '.') => Some(vec![("。", Stop)]),
    ("", '?') => Some(vec![("？", Stop)]),
    ("", '/') => Some(vec![("/", Stop)]),
    ("", ';') => Some(vec![(";", Stop)]),
    ("", ':') => Some(vec![(":", Stop)]),
    ("", '\'') => Some(vec![("\\", Stop)]),
    ("", '`') => Some(vec![("`", Stop)]),
    ("", '~') => Some(vec![("~", Stop)]),

    ("", '!') => Some(vec![("！", Stop)]),
    ("", '@') => Some(vec![("@", Stop)]),
    ("", '#') => Some(vec![("#", Stop)]),
    ("", '$') => Some(vec![("$", Stop)]),
    ("", '%') => Some(vec![("%", Stop)]),
    ("", '^') => Some(vec![("^", Stop)]),
    ("", '&') => Some(vec![("&", Stop)]),
    ("", '*') => Some(vec![("*", Stop)]),

    ("", '\\') => Some(vec![("\\", Stop)]),
    ("", '|') => Some(vec![("|", Stop)]),

    ("", '(') => Some(vec![("（", Stop)]),
    ("", ')') => Some(vec![("）", Stop)]),
    ("", '[') => Some(vec![("「", Stop)]),
    ("", ']') => Some(vec![("」", Stop)]),
    ("", '{') => Some(vec![("{", Stop)]),
    ("", '}') => Some(vec![("}", Stop)]),
    ("", '<') => Some(vec![("<", Stop)]),
    ("", '>') => Some(vec![(">", Stop)]),

    // 複合記号類
    ("z", ' ') => Some(vec![("　", Stop)]),

    ("z", '[') => Some(vec![("『", Stop)]),
    ("z", ']') => Some(vec![("』", Stop)]),

    ("z", ',') => Some(vec![("‥", Stop)]),
    ("z", '.') => Some(vec![("…", Stop)]),
    ("z", '/') => Some(vec![("・", Stop)]),

    ("z", 'h') => Some(vec![("←", Stop)]),
    ("z", 'j') => Some(vec![("↓", Stop)]),
    ("z", 'k') => Some(vec![("↑", Stop)]),
    ("z", 'l') => Some(vec![("→", Stop)]),

    _ => None,
  }?;

  Some(
    pairs
      .into_iter()
      .map(|(c, state)| BufferPair::new(Hiragana, c, state))
      .collect::<Vec<BufferPair>>(),
  )
}
