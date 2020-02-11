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

  pub fn push_buffer<S: Into<String>>(&mut self, buffer: S) {
    self.stopped_buffer = Some(match &self.stopped_buffer {
      None => buffer.into(),
      Some(buf) => buf.to_owned() + &buffer.into(),
    })
  }

  pub fn pop_stopped_buffer(&mut self) {
    self.stopped_buffer = match &self.stopped_buffer {
      None => None,
      Some(buf) => {
        let mut buf = buf.clone();
        buf.pop();

        match buf.is_empty() {
          true => None,
          false => Some(buf),
        }
      }
    }
  }

  pub fn stopped_buffer(&self) -> Option<String> {
    self.stopped_buffer.clone()
  }

  pub fn dictionary_updates(&self) -> &Vec<DictionaryEntry> {
    &self.dictionary_updates
  }

  pub fn push_dictionary_updates(&mut self, updates: &Vec<DictionaryEntry>) {
    updates
      .iter()
      .for_each(|item| self.dictionary_updates.push(item.clone()))
  }

  pub fn merge_result(&mut self, result: &CompositionResult) {
    result.stopped_buffer().map(|buf| self.push_buffer(buf));

    self.push_dictionary_updates(result.dictionary_updates());
  }

  pub fn clear_stopped_buffer(&mut self) {
    self.stopped_buffer = None
  }
}
