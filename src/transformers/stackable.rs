use super::Transformable;

pub trait Stackable {
  fn push(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable>;
  fn pop(&self) -> (Box<dyn Transformable>, Option<Box<dyn Transformable>>);
  fn replace_last_element(&self, item: Box<dyn Transformable>) -> Box<dyn Transformable>;
  fn stack(&self) -> Vec<Box<dyn Transformable>>;
  fn is_all_stopped(&self) -> bool {
    self.stack().iter().all(|item| item.is_stopped())
  }
}
