mod composition;
mod config;
mod dictionary;
mod keyboards;
mod tests;
mod transformers;

use composition::Composition;
use config::Config;
use dictionary::Dictionary;
use std::collections::HashSet;
use std::rc::Rc;
use transformers::TransformerTypes;

pub struct RSKK {
    config: Rc<Config>,
    compositions: Vec<Composition>,
    default_composition_type: TransformerTypes,
    dictionary: Dictionary,
}

impl RSKK {
    pub fn new(default_composition_type: TransformerTypes) -> Self {
        RSKK {
            config: Rc::new(Config::default_config()),
            compositions: vec![],
            default_composition_type,
            dictionary: Dictionary::new(set![]),
        }
    }

    pub fn parse_dictionary(&mut self, dic: &str) {
        self.dictionary = Dictionary::parse(dic);
    }

    pub fn start_composition(&mut self) -> &mut Composition {
        self.start_composition_as(self.default_composition_type)
    }

    pub fn start_composition_as(&mut self, composition_type: TransformerTypes) -> &mut Composition {
        self.compositions
            .push(Composition::new(Rc::clone(&self.config), composition_type));

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

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::tests::helpers::str_to_key_code_vector;

    #[test]
    fn it_works() {
        let mut skk = RSKK::new(TransformerTypes::Direct);
        let composition = skk.start_composition();
        composition.push_key_events(&str_to_key_code_vector("abc"));
        assert_eq!(composition.display_string(), "abc");

        let composition = skk.start_composition();
        composition.push_key_events(&str_to_key_code_vector("[down:shift]a[up:shift]b"));
        assert_eq!(composition.display_string(), "Ab");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("a"));
        assert_eq!(composition.display_string(), "あ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.push_key_events(&str_to_key_code_vector("ai"));
        assert_eq!(composition.display_string(), "あい");

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
        composition.push_key_events(&str_to_key_code_vector("a[down:ctrl]j[up:ctrl]ala"));
        assert_eq!(composition.display_string(), "aあa");
    }
}
