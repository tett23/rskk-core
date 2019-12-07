use super::{
  AsTransformerTrait, CanceledTransformer, Config, ContinuousTransformer, Displayable,
  OkuriCompletedTransformer, Stackable, StoppedTransformer, Transformable, TransformerState,
  TransformerTypes, WithConfig,
};
use crate::keyboards::KeyCode;
use crate::set;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct YomiTransformer {
  config: Config,
  transformer_type: TransformerTypes,
  pair: (Box<dyn Transformable>, Option<Box<dyn Transformable>>),
}

impl YomiTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    YomiTransformer {
      config: config.clone(),
      transformer_type,
      pair: (
        Box::new(ContinuousTransformer::new(config, transformer_type)),
        None,
      ),
    }
  }
}

impl WithConfig for YomiTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl TransformerState for YomiTransformer {
  fn is_stopped(&self) -> bool {
    false
  }
}

impl Transformable for YomiTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Yomi
  }

  fn try_change_transformer(
    &self,
    pressing_keys: &HashSet<KeyCode>,
    last_key_code: &KeyCode,
  ) -> Option<Box<dyn Transformable>> {
    match &self.pair {
      (_, None) => {
        let transformer_type = self
          .config
          .key_config()
          .try_change_transformer(&set![TransformerTypes::Henkan], pressing_keys);
        match transformer_type {
          Some(TransformerTypes::Henkan) => {
            let ret = self.clone();
            let okuri = self.transformer_type.to_transformer(self.config());
            let okuri = if let Some(character) = last_key_code.printable_key() {
              okuri.push_character(character)
            } else {
              okuri
            };

            Some(ret.push(okuri))
          }
          _ => None,
        }
      }
      (_, Some(_)) => None,
    }
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let new_transformer = self.send_target().push_character(character);

    match (new_transformer.is_stopped(), &self.pair) {
      (false, _) => self.replace_last_element(new_transformer),
      (true, (_, None)) => Box::new(StoppedTransformer::new(
        self.config(),
        self.replace_last_element(new_transformer).buffer_content(),
      )),
      (true, (_, Some(_))) if new_transformer.transformer_type() == TransformerTypes::Canceled => {
        self.pop().0
      }
      (true, (_, Some(_))) => Box::new(OkuriCompletedTransformer::new(
        self.config(),
        self.pair.0.buffer_content(),
        new_transformer.buffer_content(),
      )),
    }
  }

  fn transformer_updated(&self, new_transformer: Box<dyn Transformable>) -> Box<dyn Transformable> {
    match (new_transformer.is_stopped(), &self.pair) {
      (true, (_, None)) => new_transformer,
      (true, (_, Some(_))) if new_transformer.transformer_type() == TransformerTypes::Canceled => {
        self.pop().0
      }
      (true, (_, Some(_))) if new_transformer.transformer_type() == TransformerTypes::Stopped => {
        Box::new(StoppedTransformer::new(
          self.config(),
          self.buffer_content(),
        ))
      }
      (true, (_, Some(_))) => unreachable!(),
      (false, _) => self.replace_last_element(new_transformer),
    }
  }
}

impl Displayable for YomiTransformer {
  fn buffer_content(&self) -> String {
    match &self.pair {
      (yomi, None) => yomi.buffer_content(),
      (yomi, Some(okuri)) => yomi.buffer_content() + &okuri.buffer_content(),
    }
  }

  fn display_string(&self) -> String {
    match &self.pair {
      (yomi, None) => "▽".to_string() + &yomi.buffer_content(),
      (yomi, Some(okuri)) => {
        "▽".to_string() + &yomi.buffer_content() + "*" + &okuri.buffer_content()
      }
    }
  }
}

impl AsTransformerTrait for YomiTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }

  fn send_target(&self) -> Box<dyn Transformable> {
    match &self.pair {
      (yomi, None) => yomi.clone(),
      (_, Some(okuri)) => okuri.clone(),
    }
  }
}

impl Stackable for YomiTransformer {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    let mut ret = self.clone();

    ret.pair.1 = Some(item);

    Box::new(ret)
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    match &self.pair {
      (yomi, None) => (
        Box::new(CanceledTransformer::new(self.config())),
        Some(yomi.clone()),
      ),
      (_, Some(okuri)) => {
        let mut ret = self.clone();
        ret.pair.1 = None;

        (Box::new(ret), Some(okuri.clone()))
      }
    }
  }

  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable> {
    match &self.pair {
      (_, None) => {
        let mut ret = self.clone();
        ret.pair.0 = item;

        Box::new(ret)
      }
      (_, Some(_)) => {
        let mut ret = self.clone();
        ret.pair.1 = Some(item);

        Box::new(ret)
      }
    }
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

    let items = tds![conf, YomiTransformer, Hiragana;
      ["hiragana", "▽ひらがな", Yomi],
      ["hiragana\n", "ひらがな", Stopped],
      ["oku[escape]", "", Canceled],
      ["okuR", "▽おく*r", Yomi],
      ["okuR[escape]", "▽おく", Yomi],
      ["okuR\n", "▽おく", Yomi],
      ["okuRi", "おくり", OkuriCompleted],
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}
