use crate::keycodes::KeyCode;
use crate::transformers::Transformer;

pub struct DirectTransformer {
  buffer: String,
  is_stopped: bool,
}

impl DirectTransformer {
  pub fn new() -> Self {
    DirectTransformer {
      buffer: "".to_string(),
      is_stopped: false,
    }
  }
}

impl Transformer for DirectTransformer {
  fn is_stopped(&self) -> bool {
    self.is_stopped
  }

  fn push(&mut self, key: &KeyCode, shift: bool) {
    if self.is_stopped {
      return;
    }

    let character = match (key, shift) {
      (KeyCode::KeyA, true) => Some("A"),
      (KeyCode::KeyA, false) => Some("a"),
      (KeyCode::KeyB, true) => Some("B"),
      (KeyCode::KeyB, false) => Some("b"),
      (KeyCode::KeyC, true) => Some("C"),
      (KeyCode::KeyC, false) => Some("c"),
      (KeyCode::KeyD, true) => Some("D"),
      (KeyCode::KeyD, false) => Some("d"),
      (KeyCode::KeyE, true) => Some("E"),
      (KeyCode::KeyE, false) => Some("e"),
      (KeyCode::KeyF, true) => Some("F"),
      (KeyCode::KeyF, false) => Some("f"),
      (KeyCode::KeyG, true) => Some("G"),
      (KeyCode::KeyG, false) => Some("g"),
      (KeyCode::KeyH, true) => Some("H"),
      (KeyCode::KeyH, false) => Some("h"),
      (KeyCode::KeyI, true) => Some("I"),
      (KeyCode::KeyI, false) => Some("i"),
      (KeyCode::KeyJ, true) => Some("J"),
      (KeyCode::KeyJ, false) => Some("j"),
      (KeyCode::KeyK, true) => Some("K"),
      (KeyCode::KeyK, false) => Some("k"),
      (KeyCode::KeyL, true) => Some("L"),
      (KeyCode::KeyL, false) => Some("l"),
      (KeyCode::KeyM, true) => Some("M"),
      (KeyCode::KeyM, false) => Some("m"),
      (KeyCode::KeyN, true) => Some("N"),
      (KeyCode::KeyN, false) => Some("n"),
      (KeyCode::KeyO, true) => Some("O"),
      (KeyCode::KeyO, false) => Some("o"),
      (KeyCode::KeyP, true) => Some("P"),
      (KeyCode::KeyP, false) => Some("p"),
      (KeyCode::KeyQ, true) => Some("Q"),
      (KeyCode::KeyQ, false) => Some("q"),
      (KeyCode::KeyR, true) => Some("R"),
      (KeyCode::KeyR, false) => Some("r"),
      (KeyCode::KeyS, true) => Some("S"),
      (KeyCode::KeyS, false) => Some("s"),
      (KeyCode::KeyT, true) => Some("T"),
      (KeyCode::KeyT, false) => Some("t"),
      (KeyCode::KeyU, true) => Some("U"),
      (KeyCode::KeyU, false) => Some("u"),
      (KeyCode::KeyW, true) => Some("W"),
      (KeyCode::KeyW, false) => Some("w"),
      (KeyCode::KeyX, true) => Some("X"),
      (KeyCode::KeyX, false) => Some("x"),
      (KeyCode::KeyY, true) => Some("Y"),
      (KeyCode::KeyY, false) => Some("y"),
      (KeyCode::KeyZ, true) => Some("Z"),
      (KeyCode::KeyZ, false) => Some("z"),
      (KeyCode::Key1, true) => Some("!"),
      (KeyCode::Key1, false) => Some("1"),
      (KeyCode::Key2, true) => Some("@"),
      (KeyCode::Key2, false) => Some("2"),
      (KeyCode::Key3, true) => Some("#"),
      (KeyCode::Key3, false) => Some("3"),
      (KeyCode::Key4, true) => Some("$"),
      (KeyCode::Key4, false) => Some("4"),
      (KeyCode::Key5, true) => Some("%"),
      (KeyCode::Key5, false) => Some("5"),
      (KeyCode::Key6, true) => Some("^"),
      (KeyCode::Key6, false) => Some("6"),
      (KeyCode::Key7, true) => Some("&"),
      (KeyCode::Key7, false) => Some("7"),
      (KeyCode::Key8, true) => Some("*"),
      (KeyCode::Key8, false) => Some("8"),
      (KeyCode::Key9, true) => Some("("),
      (KeyCode::Key9, false) => Some("9"),
      (KeyCode::Key0, true) => Some(")"),
      (KeyCode::Key0, false) => Some("0"),
      _ => None,
    };

    if let Some(c) = character {
      self.is_stopped = true;
      self.buffer.push_str(c);
    };
  }

  fn exit(&mut self) -> String {
    self.is_stopped = true;

    std::mem::replace(&mut self.buffer, "".to_string())
  }

  fn buffer_content(&self) -> String {
    self.buffer.clone()
  }

  fn display_string(&self) -> String {
    self.buffer.clone()
  }
}
