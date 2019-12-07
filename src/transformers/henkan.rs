use super::{
  AsTransformerTrait, Config, Displayable, MetaKey, SelectCandidate, Stackable, Stopped,
  Transformable, TransformerState, TransformerTypes, UnknownWord, WithConfig, Word,
  YomiTransformer,
};
use crate::keyboards::KeyCode;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct HenkanTransformer {
  config: Config,
  transformer_type: TransformerTypes,
  stack: Vec<Box<dyn Transformable>>,
}

impl HenkanTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    HenkanTransformer {
      config: config.clone(),
      transformer_type: transformer_type.clone(),
      stack: vec![Box::new(YomiTransformer::new(config, transformer_type))],
    }
  }
}

impl WithConfig for HenkanTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for HenkanTransformer {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for HenkanTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Henkan
  }

  fn try_change_transformer(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    self
      .send_target()
      .try_change_transformer(pressing_keys, last_key_code)
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_character(character);

    self.replace_last_element(new_transformer)
  }

  fn push_meta_key(&self, key_code: &KeyCode) -> Box<dyn Transformable> {
    let target = self.send_target();

    let new_transformer = match key_code {
      KeyCode::Meta(MetaKey::Escape) => target.push_escape(),
      KeyCode::PrintableMeta(MetaKey::Enter, _) | KeyCode::Meta(MetaKey::Enter) => {
        target.push_enter()
      }
      KeyCode::PrintableMeta(MetaKey::Space, _) | KeyCode::Meta(MetaKey::Space) => {
        self.push_space()
      }
      KeyCode::PrintableMeta(MetaKey::Backspace, _) | KeyCode::Meta(MetaKey::Backspace) => {
        target.push_backspace()
      }
      KeyCode::PrintableMeta(MetaKey::Delete, _) | KeyCode::Meta(MetaKey::Delete) => {
        target.push_delete()
      }
      KeyCode::PrintableMeta(MetaKey::Tab, _) | KeyCode::Meta(MetaKey::Tab) => target.push_tab(),
      KeyCode::Meta(MetaKey::ArrowRight) => target.push_arrow_right(),
      KeyCode::Meta(MetaKey::ArrowDown) => target.push_arrow_down(),
      KeyCode::Meta(MetaKey::ArrowLeft) => target.push_arrow_left(),
      KeyCode::Meta(MetaKey::ArrowUp) => target.push_arrow_up(),
      _ => return self.as_trait(),
    };

    self.transformer_updated(new_transformer)
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    let buf = self.buffer_content();
    match self.config.dictionary.transform(&buf) {
      Some(dic_entry) => Box::new(SelectCandidate::new(self.config(), dic_entry)),
      None => Box::new(UnknownWord::new(self.config(), Word::new(&buf, None))),
    }
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
    if new_transformer.is_stopped() {
      return new_transformer;
    }

    self.replace_last_element(new_transformer)
  }
}

impl Displayable for HenkanTransformer {
  fn buffer_content(&self) -> String {
    self.send_target().buffer_content()
  }

  fn display_string(&self) -> String {
    self.send_target().display_string()
  }
}

impl AsTransformerTrait for HenkanTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match self.stack.last() {
      Some(tf) => tf.clone(),
      None => Box::new(Stopped::empty(self.config())),
    }
  }
}

impl Stackable for HenkanTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.push(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let mut ret = self.clone();

    let item = ret.stack.pop();

    (Box::new(ret), item)
  }

  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.stack.pop();
    ret.stack.push(item);

    Box::new(ret)
  }
}

#[cfg(test)]
mod tests {
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, HenkanTransformer, Hiragana;
      ["hiragana", "▽ひらがな", Henkan],
      ["kannji ", "▼漢字", Henkan],
      // ["kannji\n", "▼漢字", Henkan],
      // ["Kannji", "[登録: みちご]▽かんじ", UnknownWord],
      // ["Kannji ", "[登録: みちご]▼漢字", UnknownWord],
      // ["Kannji \n","[登録: みちご]漢字", UnknownWord],
      // ["Michi \nGo \n\n","未知語",Stopped]
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}
