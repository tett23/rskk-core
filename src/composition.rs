use super::keyboards::{KeyCode, KeyEvents, Keyboard, MetaKey};
use super::transformers::{Transformable, TransformerTypes};
use crate::{tf, CompositionResult, Context};

#[derive(Clone)]
pub struct Composition {
  transformer: Box<dyn Transformable>,
  base_transformer_type: TransformerTypes,
  context: Context,
  keyboard: Box<dyn Keyboard>,
  // TODO: 変更のあった辞書要素を保持できる必要あり？
  // 変更は読みと変換先だけあればいいかな。
  // 読みがマッチした要素の候補の先頭に候補を挿入する
  // 未登録の場合は新規レコードを追加
}

impl Composition {
  pub fn new(context: Context, transformer_types: TransformerTypes) -> Self {
    let keyboard = context.config().keyboard_type.to_keyboard();

    Composition {
      transformer: tf!(context.clone(), transformer_types),
      base_transformer_type: transformer_types,
      context,
      keyboard,
    }
  }

  #[cfg(test)]
  pub fn new_from_transformer(context: Context, transformer: Box<dyn Transformable>) -> Self {
    let context = context.new_empty();
    let keyboard = context.config().keyboard_type.to_keyboard();
    let mut tf = transformer;
    tf.set_context(context.clone());

    Composition {
      transformer: tf,
      base_transformer_type: TransformerTypes::Direct,
      context,
      keyboard,
    }
  }

  pub fn is_stopped(&self) -> bool {
    self.transformer.is_stopped()
  }

  pub fn is_empty(&self) -> bool {
    self.transformer.is_empty()
  }

  pub fn push_key_event(&mut self, event: &KeyEvents) -> bool {
    self.keyboard.push_event(event);

    KeyEventProcessor::new(event, &self.keyboard, &self.transformer)
      .next()
      .map(|result| match result {
        KeyEventProcessorResult::KeyProcessed(new_tf) => self.transformer = new_tf,
        KeyEventProcessorResult::TransformerChanged(new_tf) => {
          self.base_transformer_type = new_tf.transformer_type();
          self.transformer = new_tf;
        }
      })
      .and(Some(true))
      .unwrap_or(false)
  }

  pub fn next_composition(&self) -> Composition {
    Composition::new(self.context.new_empty(), self.base_transformer_type)
  }

  pub fn stopped_buffer(&self) -> String {
    self
      .transformer
      .clone_context()
      .result()
      .stopped_buffer()
      .unwrap_or(String::new())
  }

  pub fn display_string(&self) -> String {
    self.transformer.display_string()
  }

  pub fn transformer_type(&self) -> TransformerTypes {
    self.transformer.transformer_type()
  }

  pub fn child_transformer_type(&self) -> TransformerTypes {
    self.transformer.child_transformer_type()
  }

  pub fn transformer(&self) -> Box<dyn Transformable> {
    self.transformer.clone()
  }

  pub fn base_transformer_type(&self) -> TransformerTypes {
    self.base_transformer_type
  }

  pub fn result(&self) -> CompositionResult {
    self.transformer.context().result().clone()
  }

  pub fn clear_stopped_buffer(&mut self) {
    self
      .transformer
      .set_context(self.transformer.clear_stopped_buffer())
  }
}

struct KeyEventProcessor<'a> {
  event: &'a KeyEvents,
  keyboard: &'a Box<dyn Keyboard>,
  transformer: &'a Box<dyn Transformable>,
}

enum KeyEventProcessorResult {
  KeyProcessed(Box<dyn Transformable>),
  TransformerChanged(Box<dyn Transformable>),
}

impl<'a> KeyEventProcessor<'a> {
  pub fn new(
    event: &'a KeyEvents,
    keyboard: &'a Box<dyn Keyboard>,
    transformer: &'a Box<dyn Transformable>,
  ) -> Self {
    Self {
      event,
      keyboard,
      transformer,
    }
  }

  pub fn next(&self) -> Option<KeyEventProcessorResult> {
    Self::next_key_code(self.event)
      .and_then(|key| Self::is_process_key(key, self.transformer))
      .and(self.keyboard.last_character())
      .map(|key| {
        self
          .transformer
          .try_change_transformer(&self.keyboard, &key)
          .and_then(|tf| Some(KeyEventProcessorResult::TransformerChanged(tf)))
          .or_else(|| {
            self
              .transformer
              .push_key(&key)
              .map(|tf| KeyEventProcessorResult::KeyProcessed(tf))
          })
      })
      .unwrap_or(None)
  }

  fn next_key_code(event: &KeyEvents) -> Option<&KeyCode> {
    match event {
      KeyEvents::KeyDown(key) => Some(key),
      _ => None,
    }
  }

  fn is_process_key(key: &KeyCode, transformer: &Box<dyn Transformable>) -> Option<()> {
    match transformer.is_empty() {
      true => match key {
        KeyCode::Meta(MetaKey::Delete) if transformer.is_base_transformer() => None,
        KeyCode::Meta(MetaKey::Backspace) if transformer.is_base_transformer() => None,
        KeyCode::Meta(MetaKey::ArrowRight) if transformer.is_base_transformer() => None,
        KeyCode::Meta(MetaKey::ArrowDown) if transformer.is_base_transformer() => None,
        KeyCode::Meta(MetaKey::ArrowLeft) if transformer.is_base_transformer() => None,
        KeyCode::Meta(MetaKey::ArrowUp) if transformer.is_base_transformer() => None,
        KeyCode::PrintableMeta(MetaKey::Space, _) if transformer.is_base_transformer() => None,
        KeyCode::PrintableMeta(MetaKey::Enter, _) if transformer.is_base_transformer() => None,
        KeyCode::PrintableMeta(MetaKey::Tab, _) if transformer.is_base_transformer() => None,
        KeyCode::PrintableMeta(MetaKey::Shift, _) if transformer.is_base_transformer() => None,
        KeyCode::PrintableMeta(MetaKey::Ctrl, _) if transformer.is_base_transformer() => None,
        KeyCode::PrintableMeta(MetaKey::Super, _) if transformer.is_base_transformer() => None,
        _ => Some(()),
      },
      false => match key {
        _ => Some(()),
      },
    }
  }
}
