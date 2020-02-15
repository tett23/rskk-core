use crate::DictionaryEntry;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CompositionResult {
  dictionary_updates: Vec<DictionaryEntry>, // 補完に関する情報はweak refにしておいて、該当TFが消えていたら同時に参照も消滅する
  stopped_buffer: Option<String>,
}

impl CompositionResult {
  pub fn new() -> Self {
    Self {
      dictionary_updates: vec![],
      stopped_buffer: None,
    }
  }

  #[cfg(test)]
  pub fn new_from_stopped_buffer<S: Into<String>>(buffer: S) -> Self {
    Self {
      dictionary_updates: vec![],
      stopped_buffer: Some(buffer.into()),
    }
  }

  pub fn push_buffer<S: Into<String>>(&self, buffer: S) -> Self {
    let buffer = buffer.into();
    if buffer.is_empty() {
      return self.clone();
    }

    Self {
      stopped_buffer: Some(match &self.stopped_buffer {
        None => buffer,
        Some(buf) => buf.to_owned() + &buffer,
      }),
      ..self.clone()
    }
  }

  pub fn pop_stopped_buffer(&self) -> Self {
    Self {
      stopped_buffer: match &self.stopped_buffer {
        None => None,
        Some(buf) => {
          let mut buf = buf.clone();
          buf.pop();

          match buf.is_empty() {
            true => None,
            false => Some(buf),
          }
        }
      },
      ..self.clone()
    }
  }

  pub fn stopped_buffer(&self) -> Option<String> {
    self.stopped_buffer.clone()
  }

  pub fn dictionary_updates(&self) -> &Vec<DictionaryEntry> {
    &self.dictionary_updates
  }

  pub fn push_dictionary_updates(&self, updates: &Vec<DictionaryEntry>) -> Self {
    Self {
      dictionary_updates: updates
        .iter()
        .fold(self.dictionary_updates.clone(), |mut acc, item| {
          acc.push(item.clone());
          acc
        }),
      ..self.clone()
    }
  }

  pub fn merge_result(&self, result: &CompositionResult) -> Self {
    self
      .push_dictionary_updates(result.dictionary_updates())
      .push_buffer(result.stopped_buffer().unwrap_or(String::new()))
  }

  pub fn clear_stopped_buffer(&self) -> Self {
    Self {
      stopped_buffer: None,
      ..self.clone()
    }
  }
}
