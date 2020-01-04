use super::{
  AsTransformerTrait, Config, Displayable, Stackable, Transformable, TransformerTypes, WithConfig,
  YomiTransformer,
};

#[derive(Clone, Debug)]
pub struct OkuriCompletedTransformer {
  config: Config,
  transformer_type: TransformerTypes,
  yomi: String,
  okuri: String,
}

impl OkuriCompletedTransformer {
  pub fn new<S1: Into<String>, S2: Into<String>>(
    config: Config,
    transformer_type: TransformerTypes,
    yomi: S1,
    okuri: S2,
  ) -> Self {
    OkuriCompletedTransformer {
      config,
      transformer_type,
      yomi: yomi.into(),
      okuri: okuri.into(),
    }
  }
}

impl WithConfig for OkuriCompletedTransformer {
  fn config(&self) -> Config {
    self.config.clone()
  }
}

impl Transformable for OkuriCompletedTransformer {
  fn transformer_type(&self) -> TransformerTypes {
    TransformerTypes::OkuriCompleted
  }

  fn push_character(&self, _: char) -> Option<Box<dyn Transformable>> {
    None
  }
}

impl Displayable for OkuriCompletedTransformer {
  fn buffer_content(&self) -> String {
    self.yomi.clone() + &self.okuri
  }

  fn display_string(&self) -> String {
    self.buffer_content()
  }

  fn pair(&self) -> (String, Option<String>) {
    (self.yomi.clone(), Some(self.okuri.clone()))
  }
}

impl AsTransformerTrait for OkuriCompletedTransformer {
  fn as_trait(&self) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

impl Stackable for OkuriCompletedTransformer {
  fn push(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>) {
    let ret = YomiTransformer::from_pair(
      self.config(),
      self.transformer_type,
      (self.yomi.clone(), None),
    );

    (box ret, Some(box self.clone()))
  }

  fn replace_last_element(&self, _: Box<dyn Transformable>) -> Box<dyn Transformable> {
    box self.clone()
  }

  fn stack(&self) -> Vec<Box<dyn Transformable>> {
    vec![]
  }
}
