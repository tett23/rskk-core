use super::super::BufferState::*;
use super::{BufferPair, LetterType};
use LetterType::*;

pub fn convert(current_buffer: &str, character: char) -> Option<Vec<BufferPair>> {
  let character = character.to_lowercase().next()?;

  let pairs = match (current_buffer, character) {
    ("", 'a') => Some(vec![("ア", Stop)]),
    ("", 'i') => Some(vec![("イ", Stop)]),
    ("", 'u') => Some(vec![("ウ", Stop)]),
    ("", 'e') => Some(vec![("エ", Stop)]),
    ("", 'o') => Some(vec![("オ", Stop)]),

    ("", 'k') => Some(vec![("k", Continue)]),
    ("k", 'a') => Some(vec![("カ", Stop)]),
    ("k", 'i') => Some(vec![("キ", Stop)]),
    ("k", 'u') => Some(vec![("ク", Stop)]),
    ("k", 'e') => Some(vec![("ケ", Stop)]),
    ("k", 'o') => Some(vec![("コ", Stop)]),

    ("", 'g') => Some(vec![("g", Continue)]),
    ("g", 'a') => Some(vec![("ガ", Stop)]),
    ("g", 'i') => Some(vec![("ギ", Stop)]),
    ("g", 'u') => Some(vec![("グ", Stop)]),
    ("g", 'e') => Some(vec![("ゲ", Stop)]),
    ("g", 'o') => Some(vec![("ゴ", Stop)]),

    ("", 's') => Some(vec![("s", Continue)]),
    ("s", 'a') => Some(vec![("サ", Stop)]),
    ("s", 'i') => Some(vec![("シ", Stop)]),
    ("s", 'u') => Some(vec![("ス", Stop)]),
    ("s", 'e') => Some(vec![("セ", Stop)]),
    ("s", 'o') => Some(vec![("ソ", Stop)]),

    ("", 'z') => Some(vec![("z", Continue)]),
    ("z", 'a') => Some(vec![("ザ", Stop)]),
    ("z", 'i') => Some(vec![("ジ", Stop)]),
    ("z", 'u') => Some(vec![("ズ", Stop)]),
    ("z", 'e') => Some(vec![("ゼ", Stop)]),
    ("z", 'o') => Some(vec![("ゾ", Stop)]),

    ("", 't') => Some(vec![("t", Continue)]),
    ("t", 'a') => Some(vec![("タ", Stop)]),
    ("t", 'i') => Some(vec![("チ", Stop)]),
    ("t", 'u') => Some(vec![("ツ", Stop)]),
    ("t", 'e') => Some(vec![("テ", Stop)]),
    ("t", 'o') => Some(vec![("ト", Stop)]),

    ("t", 's') => Some(vec![("ts", Continue)]),
    ("ts", 'a') => Some(vec![("ツ", Stop), ("ァ", Stop)]),
    ("ts", 'i') => Some(vec![("ツ", Stop), ("ィ", Stop)]),
    ("ts", 'u') => Some(vec![("ツ", Stop)]),
    ("ts", 'e') => Some(vec![("ツ", Stop), ("ゥ", Stop)]),
    ("ts", 'o') => Some(vec![("ツ", Stop), ("ォ", Stop)]),

    ("", 'd') => Some(vec![("d", Continue)]),
    ("d", 'a') => Some(vec![("ダ", Stop)]),
    ("d", 'i') => Some(vec![("ヂ", Stop)]),
    ("d", 'u') => Some(vec![("ヅ", Stop)]),
    ("d", 'e') => Some(vec![("デ", Stop)]),
    ("d", 'o') => Some(vec![("ド", Stop)]),
    ("d", 'h') => Some(vec![("dh", Continue)]),
    ("dh", 'a') => Some(vec![("デ", Stop), ("ャ", Stop)]),
    ("dh", 'i') => Some(vec![("デ", Stop), ("ィ", Stop)]),
    ("dh", 'u') => Some(vec![("デ", Stop), ("ュ", Stop)]),
    ("dh", 'e') => Some(vec![("デ", Stop), ("ェ", Stop)]),
    ("dh", 'o') => Some(vec![("デ", Stop), ("ョ", Stop)]),
    ("d", 'y') => Some(vec![("dy", Continue)]),
    ("dy", 'a') => Some(vec![("ヂ", Stop), ("ャ", Stop)]),
    ("dy", 'i') => Some(vec![("ヂ", Stop), ("ィ", Stop)]),
    ("dy", 'u') => Some(vec![("ヂ", Stop), ("ュ", Stop)]),
    ("dy", 'e') => Some(vec![("ヂ", Stop), ("ェ", Stop)]),
    ("dy", 'o') => Some(vec![("ヂ", Stop), ("ョ", Stop)]),

    ("", 'c') => Some(vec![("c", Continue)]),
    ("c", 'h') => Some(vec![("ch", Continue)]),
    ("ch", 'a') => Some(vec![("チ", Stop), ("ァ", Stop)]),
    ("ch", 'i') => Some(vec![("チ", Stop)]),
    ("ch", 'u') => Some(vec![("チ", Stop), ("ュ", Stop)]),
    ("ch", 'e') => Some(vec![("チ", Stop), ("ェ", Stop)]),
    ("ch", 'o') => Some(vec![("チ", Stop), ("ョ", Stop)]),

    ("c", 'y') => Some(vec![("cy", Continue)]),
    ("cy", 'a') => Some(vec![("チ", Stop), ("ァ", Stop)]),
    ("cy", 'i') => Some(vec![("チ", Stop), ("ィ", Stop)]),
    ("cy", 'u') => Some(vec![("チ", Stop), ("ュ", Stop)]),
    ("cy", 'e') => Some(vec![("チ", Stop), ("ェ", Stop)]),
    ("cy", 'o') => Some(vec![("チ", Stop), ("ョ", Stop)]),

    ("", 'n') => Some(vec![("n", Continue)]),
    ("n", 'a') => Some(vec![("ナ", Stop)]),
    ("n", 'i') => Some(vec![("ニ", Stop)]),
    ("n", 'u') => Some(vec![("ヌ", Stop)]),
    ("n", 'e') => Some(vec![("ネ", Stop)]),
    ("n", 'o') => Some(vec![("ノ", Stop)]),
    ("n", 'n') => Some(vec![("ン", Stop)]),

    ("", 'h') => Some(vec![("h", Continue)]),
    ("h", 'a') => Some(vec![("ハ", Stop)]),
    ("h", 'i') => Some(vec![("ヒ", Stop)]),
    ("h", 'u') => Some(vec![("フ", Stop)]),
    ("h", 'e') => Some(vec![("ヘ", Stop)]),
    ("h", 'o') => Some(vec![("ホ", Stop)]),

    ("", 'p') => Some(vec![("p", Continue)]),
    ("p", 'a') => Some(vec![("パ", Stop)]),
    ("p", 'i') => Some(vec![("ピ", Stop)]),
    ("p", 'u') => Some(vec![("プ", Stop)]),
    ("p", 'e') => Some(vec![("ペ", Stop)]),
    ("p", 'o') => Some(vec![("ポ", Stop)]),

    ("", 'b') => Some(vec![("b", Continue)]),
    ("b", 'a') => Some(vec![("バ", Stop)]),
    ("b", 'i') => Some(vec![("ビ", Stop)]),
    ("b", 'u') => Some(vec![("ブ", Stop)]),
    ("b", 'e') => Some(vec![("ベ", Stop)]),
    ("b", 'o') => Some(vec![("ボ", Stop)]),

    ("", 'f') => Some(vec![("f", Continue)]),
    ("f", 'a') => Some(vec![("フ", Stop), ("ァ", Stop)]),
    ("f", 'i') => Some(vec![("フ", Stop), ("ィ", Stop)]),
    ("f", 'u') => Some(vec![("フ", Stop)]),
    ("f", 'e') => Some(vec![("フ", Stop), ("ェ", Stop)]),
    ("f", 'o') => Some(vec![("フ", Stop), ("ォ", Stop)]),

    ("", 'm') => Some(vec![("m", Continue)]),
    ("m", 'a') => Some(vec![("マ", Stop)]),
    ("m", 'i') => Some(vec![("ミ", Stop)]),
    ("m", 'u') => Some(vec![("ム", Stop)]),
    ("m", 'e') => Some(vec![("メ", Stop)]),
    ("m", 'o') => Some(vec![("モ", Stop)]),

    ("", 'y') => Some(vec![("y", Continue)]),
    ("y", 'a') => Some(vec![("ヤ", Stop)]),
    ("y", 'i') => Some(vec![("イ", Stop)]),
    ("y", 'u') => Some(vec![("ユ", Stop)]),
    ("y", 'e') => Some(vec![("ェ", Stop), ("ェ", Stop)]),
    ("y", 'o') => Some(vec![("ヨ", Stop)]),

    ("", 'j') => Some(vec![("j", Continue)]),
    ("j", 'a') => Some(vec![("ジ", Stop), ("ャ", Stop)]),
    ("j", 'i') => Some(vec![("ジ", Stop)]),
    ("j", 'u') => Some(vec![("ジ", Stop), ("ュ", Stop)]),
    ("j", 'e') => Some(vec![("ジ", Stop), ("ェ", Stop)]),
    ("j", 'o') => Some(vec![("ジ", Stop), ("ョ", Stop)]),

    ("j", 'y') => Some(vec![("jy", Continue)]),
    ("jy", 'a') => Some(vec![("ジ", Stop), ("ャ", Stop)]),
    ("jy", 'i') => Some(vec![("ジ", Stop), ("ィ", Stop)]),
    ("jy", 'u') => Some(vec![("ジ", Stop), ("ュ", Stop)]),
    ("jy", 'e') => Some(vec![("ジ", Stop), ("ェ", Stop)]),
    ("jy", 'o') => Some(vec![("ジ", Stop), ("ョ", Stop)]),

    ("", 'r') => Some(vec![("r", Continue)]),
    ("r", 'a') => Some(vec![("ラ", Stop)]),
    ("r", 'i') => Some(vec![("リ", Stop)]),
    ("r", 'u') => Some(vec![("ル", Stop)]),
    ("r", 'e') => Some(vec![("レ", Stop)]),
    ("r", 'o') => Some(vec![("ロ", Stop)]),

    ("", 'w') => Some(vec![("w", Continue)]),
    ("w", 'a') => Some(vec![("ワ", Stop)]),
    ("w", 'i') => Some(vec![("ウ", Stop), ("ィ", Stop)]),
    ("w", 'u') => Some(vec![("ウ", Stop)]),
    ("w", 'e') => Some(vec![("ウ", Stop), ("ェ", Stop)]),
    ("w", 'o') => Some(vec![("ヲ", Stop)]),

    ("", 'v') => Some(vec![("v", Continue)]),
    ("v", 'a') => Some(vec![("ヴ", Stop), ("ァ", Stop)]),
    ("v", 'i') => Some(vec![("ヴ", Stop), ("ィ", Stop)]),
    ("v", 'u') => Some(vec![("ヴ", Stop)]),
    ("v", 'e') => Some(vec![("ヴ", Stop), ("ェ", Stop)]),
    ("v", 'o') => Some(vec![("ヴ", Stop)]),

    ("", 'x') => Some(vec![("x", Continue)]),
    ("x", 'a') => Some(vec![("ァ", Stop)]),
    ("x", 'i') => Some(vec![("ィ", Stop)]),
    ("x", 'u') => Some(vec![("ゥ", Stop)]),
    ("x", 'e') => Some(vec![("ェ", Stop)]),
    ("x", 'o') => Some(vec![("ォ", Stop)]),

    ("x", 'y') => Some(vec![("xy", Continue)]),
    ("xy", 'a') => Some(vec![("ャ", Stop)]),
    ("xy", 'i') => Some(vec![("イ", Stop)]),
    ("xy", 'u') => Some(vec![("ュ", Stop)]),
    ("xy", 'e') => Some(vec![("エ", Stop)]),
    ("xy", 'o') => Some(vec![("ョ", Stop)]),

    // 「tte」 -> 「って」のような促音のルール
    // TODO: 「kkkk」 -> 「っっっk」のような連続した促音のルールが未実装
    ("w", 'w') => Some(vec![("ッ", Stop), ("w", Continue)]),
    ("r", 'r') => Some(vec![("ッ", Stop), ("r", Continue)]),
    ("t", 't') => Some(vec![("ッ", Stop), ("t", Continue)]),
    ("y", 'y') => Some(vec![("ッ", Stop), ("y", Continue)]),
    ("p", 'p') => Some(vec![("ッ", Stop), ("p", Continue)]),
    ("s", 's') => Some(vec![("ッ", Stop), ("s", Continue)]),
    ("d", 'd') => Some(vec![("ッ", Stop), ("d", Continue)]),
    ("g", 'g') => Some(vec![("ッ", Stop), ("g", Continue)]),
    ("h", 'h') => Some(vec![("ッ", Stop), ("h", Continue)]),
    ("j", 'j') => Some(vec![("ッ", Stop), ("j", Continue)]),
    ("k", 'k') => Some(vec![("ッ", Stop), ("k", Continue)]),
    ("l", 'l') => Some(vec![("ッ", Stop), ("l", Continue)]),
    ("z", 'z') => Some(vec![("ッ", Stop), ("z", Continue)]),
    ("x", 'x') => Some(vec![("ッ", Stop), ("x", Continue)]),
    ("c", 'c') => Some(vec![("ッ", Stop), ("c", Continue)]),
    ("v", 'v') => Some(vec![("ッ", Stop), ("v", Continue)]),
    ("b", 'b') => Some(vec![("ッ", Stop), ("b", Continue)]),
    ("m", 'm') => Some(vec![("ッ", Stop), ("m", Continue)]),

    // 「kanji」を「かんじ」にするように、「n」での「ん」の入力ルール
    ("n", 'w') => Some(vec![("ン", Stop), ("w", Continue)]),
    ("n", 'r') => Some(vec![("ン", Stop), ("r", Continue)]),
    ("n", 't') => Some(vec![("ン", Stop), ("t", Continue)]),
    ("n", 'y') => Some(vec![("ン", Stop), ("y", Continue)]),
    ("n", 'p') => Some(vec![("ン", Stop), ("p", Continue)]),
    ("n", 's') => Some(vec![("ン", Stop), ("s", Continue)]),
    ("n", 'd') => Some(vec![("ン", Stop), ("d", Continue)]),
    ("n", 'g') => Some(vec![("ン", Stop), ("g", Continue)]),
    ("n", 'h') => Some(vec![("ン", Stop), ("h", Continue)]),
    ("n", 'j') => Some(vec![("ン", Stop), ("j", Continue)]),
    ("n", 'k') => Some(vec![("ン", Stop), ("k", Continue)]),
    ("n", 'l') => Some(vec![("ン", Stop), ("l", Continue)]),
    ("n", 'z') => Some(vec![("ン", Stop), ("z", Continue)]),
    ("n", 'x') => Some(vec![("ン", Stop), ("x", Continue)]),
    ("n", 'c') => Some(vec![("ン", Stop), ("c", Continue)]),
    ("n", 'v') => Some(vec![("ン", Stop), ("v", Continue)]),
    ("n", 'b') => Some(vec![("ン", Stop), ("b", Continue)]),
    ("n", 'm') => Some(vec![("ン", Stop), ("m", Continue)]),

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
