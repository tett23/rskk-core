use super::{
  AsTransformerTrait, Config, ContinuousTransformer, Displayable, HiraganaTransformer,
  OkuriCompletedTransformer, SelectCandidateTransformer, Stackable, StoppedReason,
  StoppedTransformer, Transformable, TransformerTypes, UnknownWordTransformer, WithConfig, Word,
};
use crate::tf;
use StoppedReason::*;

#[derive(Clone, Debug)]
pub struct YomiTransformer {
  config: Config,
  current_transformer_type: TransformerTypes,
  pair: (Box<dyn Transformable>, Option<Box<dyn Transformable>>),
}

impl YomiTransformer {
  pub fn new(config: Config, transformer_type: TransformerTypes) -> Self {
    YomiTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      pair: (
        box ContinuousTransformer::new(config, transformer_type),
        None,
      ),
    }
  }

  pub fn from_pair<S: Into<String>>(
    config: Config,
    transformer_type: TransformerTypes,
    pair: (S, Option<S>),
  ) -> Self {
    YomiTransformer {
      config: config.clone(),
      current_transformer_type: transformer_type,
      pair: (
        box ContinuousTransformer::from_buffer(config.clone(), transformer_type, pair.0),
        match pair.1 {
          Some(s) => Some(box HiraganaTransformer::from_buffer(config.clone(), s)),
          None => None,
        },
      ),
    }
  }

  fn try_okuri(&self, character: char) -> Option<Box<dyn Transformable>> {
    if !character.is_uppercase() {
      return None;
    }
    if self.pair.1.is_some() {
      return None;
    }

    Some(self.push(tf!(self.config(), self.current_transformer_type)))
  }
}

impl WithConfig for YomiTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for YomiTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::Yomi
  }

  fn push_character(&self, character: char) -> Box<dyn Transformable> {
    let tf = self.try_okuri(character).unwrap_or(box self.clone());
    let new_tf = tf
      .send_target()
      .push_character(character.to_lowercase().next().unwrap());

    match (new_tf.transformer_type(), &tf.pair()) {
      (TransformerTypes::Stopped(_), (_, None)) => box StoppedTransformer::completed(
        tf.config(),
        tf.replace_last_element(new_tf).buffer_content(),
      ),
      (TransformerTypes::Stopped(Canceled), (_, Some(_))) => tf.pop().0,
      (TransformerTypes::Stopped(Compleated), (yomi, Some(_))) => {
        box OkuriCompletedTransformer::new(
          tf.config(),
          self.current_transformer_type,
          yomi,
          new_tf.buffer_content(),
        )
      }
      (_, _) => tf.replace_last_element(new_tf),
    }
  }

  fn push_escape(&self) -> Box<dyn Transformable> {
    match &self.pair {
      (_, None) => Box::new(StoppedTransformer::canceled(self.config())),
      (_, Some(_)) => self.pop().0,
    }
  }

  fn push_space(&self) -> Box<dyn Transformable> {
    let buf = self.buffer_content();
    match self.config.dictionary.transform(&buf) {
      Some(dic_entry) => Box::new(SelectCandidateTransformer::new(
        self.config(),
        dic_entry,
        None,
      )),
      None => Box::new(UnknownWordTransformer::new(
        self.config(),
        Word::new(&buf, None),
      )),
    }
  }

  fn push_enter(&self) -> Box<dyn Transformable> {
    match &self.pair {
      (yomi, None) => Box::new(StoppedTransformer::completed(
        self.config(),
        yomi.buffer_content(),
      )),
      (_, Some(okuri)) => match okuri.push_enter().is_canceled() {
        true => self.pop().0,
        false => self.as_trait(),
      },
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

  fn pair(&self) -> (String, Option<String>) {
    match &self.pair {
      (yomi, Some(okuri)) => (yomi.buffer_content(), Some(okuri.buffer_content())),
      (yomi, None) => (yomi.buffer_content(), None),
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
        Box::new(StoppedTransformer::canceled(self.config())),
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

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    let mut ret = vec![self.pair.0.clone()];
    if let Some(yomi) = &self.pair.1 {
      ret.push(yomi.clone());
    };

    ret
  }
}

#[cfg(test)]
mod tests {
  use crate::tds;
  use crate::tests::{dummy_conf, test_transformer};
  use crate::transformers::StoppedReason::*;
  use crate::transformers::TransformerTypes::*;

  #[test]
  fn it_works() {
    let conf = dummy_conf();

    let items = tds![conf, YomiTransformer, Hiragana;
      ["hiragana", "▽ひらがな", Yomi],
      ["hiragana\n", "ひらがな", Stopped(Compleated)],
      ["oku[escape]", "", Stopped(Canceled)],
      ["okuR", "▽おく*r", Yomi],
      ["okuR[escape]", "▽おく", Yomi],
      ["okuR\n", "▽おく", Yomi],
      ["okuRi", "おくり", OkuriCompleted],
      ["kannji ", "▼漢字", SelectCandidate],
      ["kannji [escape]", "", Stopped(Canceled)],
      ["michigo ", "[登録: みちご]", UnknownWord],
    ];
    test_transformer(items);

    // TODO: カタカナ時のテスト
  }
}
