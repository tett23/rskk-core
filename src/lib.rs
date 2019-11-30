mod composition;
mod config;
mod dictionary;
mod keyboards;
mod tests;
mod transformers;

use composition::Composition;
use std::collections::HashSet;
use std::rc::Rc;
use transformers::TransformerTypes;

pub use config::Config;
pub use dictionary::{Dictionary, DictionaryEntry};

pub struct RSKK {
    config: Rc<Config>,
    dictionary: Rc<Dictionary>,
    compositions: Vec<Composition>,
    default_composition_type: TransformerTypes,
}

impl RSKK {
    pub fn new(default_composition_type: TransformerTypes) -> Self {
        RSKK {
            config: Rc::new(Config::default_config()),
            dictionary: Rc::new(Dictionary::new(set![])),
            compositions: vec![],
            default_composition_type,
        }
    }

    pub fn parse_dictionary(&mut self, dic: &str) {
        self.dictionary = Rc::new(Dictionary::parse(dic));
    }

    pub fn start_composition(&mut self) -> &mut Composition {
        self.start_composition_as(self.default_composition_type)
    }

    pub fn start_composition_as(&mut self, composition_type: TransformerTypes) -> &mut Composition {
        self.compositions.push(Composition::new(
            self.config.clone(),
            self.dictionary.clone(),
            composition_type,
        ));

        self.compositions.last_mut().unwrap()
    }
}

#[macro_export]
macro_rules! set {
  ( $( $x:expr ),* ) => {
      {
          #[allow(unused_mut)]
          let mut temp_set = HashSet::new();
          $(
              temp_set.insert($x);
          )*
          temp_set
      }
  };
}

#[macro_export]
macro_rules! combo {
  ( $( $x:expr ),* ) => {
      {
          #[allow(unused_mut)]
          let mut temp_set = HashSet::new();
          $(
              temp_set.insert($x);
          )*

          KeyCombination::new(temp_set)
      }
  };
}

#[macro_export]
macro_rules! combos {
  ( $( $x:expr ),* ) => {
      {
          #[allow(unused_mut)]
          let mut temp_set = HashSet::new();
          $(
              temp_set.insert($x);
          )*

          KeyCombinations::new(temp_set)
      }
  };

}
#[macro_export]
macro_rules! key {
    ( $v:expr ) => {
        match $v {
            "ctrl" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Ctrl),
            "shift" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Shift),
            "alt" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Alt),
            "super" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Super),
            "enter" => {
                crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Enter, '\n')
            }
            "space" => {
                crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Space, ' ')
            }
            "tab" => crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Tab, '\t'),
            "escape" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Escape),
            "delete" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Delete),
            "backspace" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Backspace),
            "arrow_right" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowRight),
            "arrow_down" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowDown),
            "arrow_left" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowLeft),
            "arrow_up" => crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::ArrowUp),
            "null" => crate::keyboards::KeyCode::Null,
            string if string == "" => crate::keyboards::KeyCode::Null,
            string => crate::keyboards::KeyCode::Printable(string.chars().next().unwrap()),
        }
    };
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::tests::helpers::str_to_key_code_vector;

    #[test]
    fn it_works() {
        let mut skk = RSKK::new(TransformerTypes::Direct);
        skk.parse_dictionary("かんじ/漢字/");
        let composition = skk.start_composition();
        composition.push_key_events(&str_to_key_code_vector("a"));
        assert_eq!(composition.display_string(), "a");

        let composition = skk.start_composition_as(TransformerTypes::Direct);
        composition.push_key_events(&str_to_key_code_vector("A"));
        assert_eq!(composition.display_string(), "A");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("a"));
        assert_eq!(composition.display_string(), "あ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("ka"));
        assert_eq!(composition.display_string(), "か");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("ts"));
        assert_eq!(composition.display_string(), "ts");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("tsu"));
        assert_eq!(composition.display_string(), "つ");

        let composition = skk.start_composition_as(TransformerTypes::Direct);
        composition.push_key_events(&str_to_key_code_vector("[down:ctrl]j[up:ctrl]a"));
        assert_eq!(composition.display_string(), "あ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("K"));
        assert_eq!(composition.display_string(), "▽k");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("Kannji"));
        assert_eq!(composition.display_string(), "▽かんじ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("Kannji[down:enter]"));
        assert_eq!(composition.display_string(), "かんじ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("Kannji[down:space]"));
        assert_eq!(composition.display_string(), "▼漢字");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("Kannji[down:space][down:enter]"));
        assert_eq!(composition.display_string(), "漢字");
    }
}
