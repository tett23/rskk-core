#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Candidate {
  pub entry: String,
  pub annotation: Option<String>,
}

impl Candidate {
  pub fn new<S: Into<String>>(entry: S, annotation: Option<S>) -> Self {
    Candidate {
      entry: entry.into(),
      annotation: match annotation {
        Some(s) => Some(s.into()),
        None => None,
      },
    }
  }

  pub fn parse(string: &str) -> Option<Self> {
    let pair: Vec<_> = string.trim().splitn(2, ";").collect();
    let entry = pair.get(0)?.to_string();
    if entry.len() == 0 {
      return None;
    }
    let annotation = pair.get(1);

    Some(Candidate::new(
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
    let item = Candidate::parse("a;b");
    assert_eq!(
      item,
      Some(Candidate::new("a".to_string(), Some("b".to_string())))
    );

    let item = Candidate::parse("a");
    assert_eq!(item, Some(Candidate::new("a".to_string(), None)));

    let item = Candidate::parse("");
    assert_eq!(item, None);

    let item = Candidate::parse(" ");
    assert_eq!(item, None);
  }
}
