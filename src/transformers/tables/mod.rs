mod hiragana;

use super::BufferState;

pub fn hiragana_convert(
  current_buffer: &str,
  character: char,
) -> Option<Vec<(String, BufferState)>> {
  hiragana::convert(current_buffer, character)
}

pub fn convert_from_str(buf: &str) -> Option<String> {
  match buf {
    "" => None,
    _ => Some(buf),
  }
  .and_then(|buf| match buf.split_at(1) {
    ("", _) => None,
    (head, tail) => Some((
      hiragana_convert("", head.chars().next()?)?,
      tail.chars().collect::<Vec<char>>(),
    )),
  })
  .map(|(head, tail)| {
    tail.iter().fold(head, |acc, character| {
      let mut acc = acc.clone();
      match acc.pop() {
        None => None,
        Some((buf, BufferState::Continue)) => hiragana_convert(&buf, character.clone()),
        Some(item) => hiragana_convert("", character.clone()).map(|vec| {
          vec![item]
            .into_iter()
            .chain(vec.into_iter())
            .collect::<Vec<(String, BufferState)>>()
        }),
      }
      .map(|vec| acc.append(&mut vec.clone()));

      acc
    })
  })
  .map(|vec| vec.iter().fold("".to_owned(), |acc, item| acc + &item.0))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_from_str() {
    assert_eq!(convert_from_str(""), None);
    assert_eq!(convert_from_str("a"), Some("あ".to_owned()));
    assert_eq!(convert_from_str("aa"), Some("ああ".to_owned()));
    assert_eq!(convert_from_str("tte"), Some("って".to_owned()));
    assert_eq!(convert_from_str("n"), Some("n".to_owned()));
    assert_eq!(convert_from_str("nn"), Some("ん".to_owned()));
  }
}
