use super::{
  AsTransformerTrait, Config, ContinuousTransformer, Displayable, Stackable, Transformable,
  TransformerTypes, WithConfig, YomiTransformer,
};

#[derive(Clone, Debug)]
pub struct OkuriCompletedTransformer {
  config: Config,
  transformer_type: TransformerTypes,
  yomi: String,
  okuri: String,
}

impl OkuriCompletedTransformer {
  pub fn new<S: Into<String>>(
    config: Config,
    transformer_type: TransformerTypes,
    yomi: S,
    okuri: S,
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

  fn push_character(&self, _: char) -> Box<dyn Transformable> {
    Box::new(self.clone())
  }
}

impl Displayable for OkuriCompletedTransformer {
  fn buffer_content(&self) -> String {
    self.yomi.clone() + &self.okuri
  }

  fn display_string(&self) -> String {
    self.buffer_content()
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
      (
        box ContinuousTransformer::from_buffer(
          self.config(),
          self.transformer_type(),
          self.yomi.clone(),
        ),
        None,
      ),
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
