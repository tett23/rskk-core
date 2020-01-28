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

  pub fn stopped_buffer(&self) -> Option<String> {
    self.stopped_buffer.clone()
  }
}
