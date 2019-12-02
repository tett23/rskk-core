use super::Transformable;

pub trait Stackable: Transformable {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable>;
  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>);
  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable>;
}
