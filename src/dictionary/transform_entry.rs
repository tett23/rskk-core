#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct TransformEntry {
  pub entry: String,
  pub annotation: Option<String>,
}

impl TransformEntry {
  pub fn new(entry: String, annotation: Option<String>) -> Self {
    TransformEntry { entry, annotation }
  }
  pub fn parse(string: &str) -> Option<Self> {
    let pair: Vec<_> = string.trim().splitn(2, ";").collect();
    let entry = pair.get(0)?.to_string();
    if entry.len() == 0 {
      return None;
    }
    let annotation = pair.get(1);

    Some(TransformEntry::new(
      entry,
      match annotation {
        Some(v) => Some(v.to_string()),
        None => None,
      },
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let item = TransformEntry::parse("a;b");
    assert_eq!(
      item,
      Some(TransformEntry::new("a".to_string(), Some("b".to_string())))
    );

    let item = TransformEntry::parse("a");
    assert_eq!(item, Some(TransformEntry::new("a".to_string(), None)));

    let item = TransformEntry::parse("");
    assert_eq!(item, None);

    let item = TransformEntry::parse(" ");
    assert_eq!(item, None);
  }
}
