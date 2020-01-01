#![cfg(test)]

use super::str_to_key_code_vector;
use crate::composition::Composition;
use crate::transformers::{Transformable, TransformerTypes};

#[derive(Debug)]
pub struct TestData(
  pub Box<dyn Transformable>,
  pub String,
  pub String,
  pub TransformerTypes,
);

impl TestData {
  pub fn new<S: Into<String>>(
    tf: Box<dyn Transformable>,
    input: S,
    output: S,
    out_tf: TransformerTypes,
  ) -> Self {
    TestData(tf, input.into(), output.into(), out_tf)
  }
}

pub fn test_transformer(items: Vec<TestData>) {
  items.into_iter().for_each(
    |TestData(start_transformer, input, output, out_transformer)| {
      let mut composition =
        Composition::new_from_transformer(start_transformer.config(), start_transformer);
      composition.push_key_events(&str_to_key_code_vector(&input));
      assert_eq!(
        (out_transformer, output.into()),
        (composition.transformer_type(), composition.display_string()),
        "{}",
        input
      );
      println!("");
      println!("---------------");
      println!("");
    },
  );
}
