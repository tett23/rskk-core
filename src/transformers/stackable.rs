use super::Transformer;

pub trait Stackable: Transformer {
  fn push(&self, item: Box<dyn Transformer>) -> Box<dyn Transformer>;
  fn pop(&self) -> (Box<dyn Transformer>, Option<Box<dyn Transformer>>);
  fn replace_last_element(&self, item: Box<dyn Transformer>) -> Box<dyn Transformer>;
}
