use super::super::BufferState::*;
use super::{BufferPair, LetterType};
use LetterType::*;

pub fn convert(_: &str, character: char) -> Option<Vec<BufferPair>> {
  Some(vec![BufferPair::new(Direct, character.to_string(), Stop)])
}
