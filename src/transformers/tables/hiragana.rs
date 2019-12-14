use super::super::BufferState;
use super::super::BufferState::*;

pub fn convert(current_buffer: &str, character: char) -> Option<(String, BufferState)> {
  let character = character.to_lowercase().next()?;

  let pair = match (current_buffer, character) {
    ("", 'a') => Some(("あ", Stop)),
    ("", 'i') => Some(("い", Stop)),
    ("", 'u') => Some(("う", Stop)),
    ("", 'e') => Some(("え", Stop)),
    ("", 'o') => Some(("お", Stop)),

    ("", 'k') => Some(("k", Continue)),
    ("k", 'a') => Some(("か", Stop)),
    ("k", 'i') => Some(("き", Stop)),
    ("k", 'u') => Some(("く", Stop)),
    ("k", 'e') => Some(("け", Stop)),
    ("k", 'o') => Some(("こ", Stop)),

    ("", 'g') => Some(("g", Continue)),
    ("g", 'a') => Some(("が", Stop)),
    ("g", 'i') => Some(("ぎ", Stop)),
    ("g", 'u') => Some(("ぐ", Stop)),
    ("g", 'e') => Some(("げ", Stop)),
    ("g", 'o') => Some(("ご", Stop)),

    ("", 's') => Some(("s", Continue)),
    ("s", 'a') => Some(("さ", Stop)),
    ("s", 'i') => Some(("し", Stop)),
    ("s", 'u') => Some(("す", Stop)),
    ("s", 'e') => Some(("せ", Stop)),
    ("s", 'o') => Some(("そ", Stop)),

    ("", 'z') => Some(("z", Continue)),
    ("z", 'a') => Some(("ざ", Stop)),
    ("z", 'i') => Some(("じ", Stop)),
    ("z", 'u') => Some(("ず", Stop)),
    ("z", 'e') => Some(("で", Stop)),
    ("z", 'o') => Some(("ど", Stop)),

    ("", 't') => Some(("t", Continue)),
    ("t", 'a') => Some(("た", Stop)),
    ("t", 'i') => Some(("ち", Stop)),
    ("t", 'u') => Some(("つ", Stop)),
    ("t", 'e') => Some(("て", Stop)),
    ("t", 'o') => Some(("と", Stop)),

    ("t", 's') => Some(("ts", Continue)),
    ("ts", 'a') => Some(("つぁ", Stop)),
    ("ts", 'i') => Some(("つぃ", Stop)),
    ("ts", 'u') => Some(("つ", Stop)),
    ("ts", 'e') => Some(("つぇ", Stop)),
    ("ts", 'o') => Some(("つぉ", Stop)),

    ("", 'd') => Some(("d", Continue)),
    ("d", 'a') => Some(("だ", Stop)),
    ("d", 'i') => Some(("ぢ", Stop)),
    ("d", 'u') => Some(("づ", Stop)),
    ("d", 'e') => Some(("で", Stop)),
    ("d", 'o') => Some(("ど", Stop)),

    ("", 'c') => Some(("c", Continue)),
    ("c", 'h') => Some(("ch", Continue)),
    ("ch", 'a') => Some(("ちゃ", Stop)),
    ("ch", 'i') => Some(("ち", Stop)),
    ("ch", 'u') => Some(("ちゅ", Stop)),
    ("ch", 'e') => Some(("ちぇ", Stop)),
    ("ch", 'o') => Some(("ちょ", Stop)),

    ("c", 'y') => Some(("cy", Continue)),
    ("cy", 'a') => Some(("ちゃ", Stop)),
    ("cy", 'i') => Some(("ちぃ", Stop)),
    ("cy", 'u') => Some(("ちゅ", Stop)),
    ("cy", 'e') => Some(("ちぇ", Stop)),
    ("cy", 'o') => Some(("ちょ", Stop)),

    ("", 'n') => Some(("n", Continue)),
    ("n", 'a') => Some(("な", Stop)),
    ("n", 'i') => Some(("に", Stop)),
    ("n", 'u') => Some(("ぬ", Stop)),
    ("n", 'e') => Some(("ね", Stop)),
    ("n", 'o') => Some(("の", Stop)),
    ("n", 'n') => Some(("ん", Stop)),

    ("", 'h') => Some(("h", Continue)),
    ("h", 'a') => Some(("は", Stop)),
    ("h", 'i') => Some(("ひ", Stop)),
    ("h", 'u') => Some(("ふ", Stop)),
    ("h", 'e') => Some(("へ", Stop)),
    ("h", 'o') => Some(("ほ", Stop)),

    ("", 'p') => Some(("p", Continue)),
    ("p", 'a') => Some(("ぱ", Stop)),
    ("p", 'i') => Some(("ぴ", Stop)),
    ("p", 'u') => Some(("ぷ", Stop)),
    ("p", 'e') => Some(("ぺ", Stop)),
    ("p", 'o') => Some(("ぽ", Stop)),

    ("", 'b') => Some(("b", Continue)),
    ("b", 'a') => Some(("ば", Stop)),
    ("b", 'i') => Some(("び", Stop)),
    ("b", 'u') => Some(("ぶ", Stop)),
    ("b", 'e') => Some(("べ", Stop)),
    ("b", 'o') => Some(("ぼ", Stop)),

    ("", 'f') => Some(("f", Continue)),
    ("f", 'a') => Some(("ふぁ", Stop)),
    ("f", 'i') => Some(("ふぃ", Stop)),
    ("f", 'u') => Some(("ふ", Stop)),
    ("f", 'e') => Some(("ふぇ", Stop)),
    ("f", 'o') => Some(("ふぉ", Stop)),

    ("", 'm') => Some(("m", Continue)),
    ("m", 'a') => Some(("ま", Stop)),
    ("m", 'i') => Some(("み", Stop)),
    ("m", 'u') => Some(("む", Stop)),
    ("m", 'e') => Some(("め", Stop)),
    ("m", 'o') => Some(("も", Stop)),

    ("", 'y') => Some(("y", Continue)),
    ("y", 'a') => Some(("や", Stop)),
    ("y", 'i') => Some(("い", Stop)),
    ("y", 'u') => Some(("ゆ", Stop)),
    ("y", 'e') => Some(("いぇ", Stop)),
    ("y", 'o') => Some(("よ", Stop)),

    ("", 'j') => Some(("j", Continue)),
    ("j", 'a') => Some(("じゃ", Stop)),
    ("j", 'i') => Some(("じ", Stop)),
    ("j", 'u') => Some(("じゅ", Stop)),
    ("j", 'e') => Some(("じぇ", Stop)),
    ("j", 'o') => Some(("じょ", Stop)),

    ("j", 'y') => Some(("jy", Continue)),
    ("jy", 'a') => Some(("じゃ", Stop)),
    ("jy", 'i') => Some(("じぃ", Stop)),
    ("jy", 'u') => Some(("じゅ", Stop)),
    ("jy", 'e') => Some(("じぇ", Stop)),
    ("jy", 'o') => Some(("じょ", Stop)),

    ("", 'r') => Some(("r", Continue)),
    ("r", 'a') => Some(("ら", Stop)),
    ("r", 'i') => Some(("り", Stop)),
    ("r", 'u') => Some(("る", Stop)),
    ("r", 'e') => Some(("れ", Stop)),
    ("r", 'o') => Some(("ろ", Stop)),

    ("", 'w') => Some(("w", Continue)),
    ("w", 'a') => Some(("わ", Stop)),
    ("w", 'i') => Some(("うぃ", Stop)),
    ("w", 'u') => Some(("う", Stop)),
    ("w", 'e') => Some(("うぇ", Stop)),
    ("w", 'o') => Some(("を", Stop)),

    ("", 'v') => Some(("v", Continue)),
    ("v", 'a') => Some(("う゛ぁ", Stop)),
    ("v", 'i') => Some(("う゛ぃ", Stop)),
    ("v", 'u') => Some(("う゛", Stop)),
    ("v", 'e') => Some(("う゛ぇ", Stop)),
    ("v", 'o') => Some(("ぼ", Stop)),

    ("", 'x') => Some(("x", Continue)),
    ("x", 'a') => Some(("ぁ", Stop)),
    ("x", 'i') => Some(("ぃ", Stop)),
    ("x", 'u') => Some(("ぅ", Stop)),
    ("x", 'e') => Some(("ぇ", Stop)),
    ("x", 'o') => Some(("ぉ", Stop)),

    ("x", 'y') => Some(("xy", Continue)),
    ("xy", 'a') => Some(("ゃ", Stop)),
    ("xy", 'i') => Some(("い", Stop)),
    ("xy", 'u') => Some(("ゅ", Stop)),
    ("xy", 'e') => Some(("え", Stop)),
    ("xy", 'o') => Some(("ぉ", Stop)),

    // TODO: 「って」のような促音の入力がまだできない
    // TODO: kanjiをかんじにするように、nでのんの入力に未対応

    // 数字類
    ("", '1') => Some(("1", Stop)),
    ("", '2') => Some(("2", Stop)),
    ("", '3') => Some(("3", Stop)),
    ("", '4') => Some(("4", Stop)),
    ("", '5') => Some(("5", Stop)),
    ("", '6') => Some(("6", Stop)),
    ("", '7') => Some(("7", Stop)),
    ("", '8') => Some(("8", Stop)),
    ("", '9') => Some(("9", Stop)),
    ("", '0') => Some(("0", Stop)),

    // 記号類
    ("", ',') => Some(("、", Stop)),
    ("", '.') => Some(("。", Stop)),
    ("", '?') => Some(("？", Stop)),
    ("", '/') => Some(("/", Stop)),
    ("", ';') => Some((";", Stop)),
    ("", ':') => Some((":", Stop)),
    ("", '\'') => Some(("\\", Stop)),
    ("", '`') => Some(("`", Stop)),
    ("", '~') => Some(("~", Stop)),

    ("", '!') => Some(("！", Stop)),
    ("", '@') => Some(("@", Stop)),
    ("", '#') => Some(("#", Stop)),
    ("", '$') => Some(("$", Stop)),
    ("", '%') => Some(("%", Stop)),
    ("", '^') => Some(("^", Stop)),
    ("", '&') => Some(("&", Stop)),
    ("", '*') => Some(("*", Stop)),

    ("", '\\') => Some(("\\", Stop)),
    ("", '|') => Some(("|", Stop)),

    ("", '(') => Some(("（", Stop)),
    ("", ')') => Some(("）", Stop)),
    ("", '[') => Some(("「", Stop)),
    ("", ']') => Some(("」", Stop)),
    ("", '{') => Some(("{", Stop)),
    ("", '}') => Some(("}", Stop)),
    ("", '<') => Some(("<", Stop)),
    ("", '>') => Some((">", Stop)),

    // 複合記号類
    ("z", '[') => Some(("『", Stop)),
    ("z", ']') => Some(("』", Stop)),

    ("z", ',') => Some(("‥", Stop)),
    ("z", '.') => Some(("…", Stop)),
    ("z", '/') => Some(("・", Stop)),

    ("z", 'h') => Some(("←", Stop)),
    ("z", 'j') => Some(("↓", Stop)),
    ("z", 'k') => Some(("↑", Stop)),
    ("z", 'l') => Some(("→", Stop)),

    _ => None,
  };

  if let Some((c, state)) = pair {
    Some((c.to_string(), state))
  } else {
    None
  }
}
