use crate::DictionaryEntry;

#[derive(Clone, Debug)]
pub struct CompositionResult {
  dictionary_updates: Vec<DictionaryEntry>, // 補完に関する情報はweak refにしておいて、該当TFが消えていたら同時に参照も消滅する
  stopped_buffer: Option<String>,
}

impl CompositionResult {
  pub fn new() -> Self {
    CompositionResult {
      dictionary_updates: vec![],
      stopped_buffer: None,
    }
  }
}
