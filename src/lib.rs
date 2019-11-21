mod composition;
mod config;
mod keyboards;
mod tests;
mod transformers;

use composition::Composition;
use config::Config;
use std::rc::Rc;
use transformers::TransformerTypes;

pub struct RSKK {
    config: Rc<Config>,
    compositions: Vec<Composition>,
    default_composition_type: TransformerTypes,
}

impl RSKK {
    pub fn new(default_composition_type: TransformerTypes) -> Self {
        RSKK {
            config: Rc::new(Config::default_config()),
            compositions: vec![],
            default_composition_type,
        }
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
          #[warn(unused_mut)]
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
    use crate::tests::helpers;
    use keyboards::keycodes::KeyCode::*;

    #[test]
    fn it_works() {
        helpers::str_to_key_code_vector("A[up:s]b[down:s]");
        let mut skk = RSKK::new(TransformerTypes::Direct);
        let composition = skk.start_composition();
        composition.key_down(&KeyA);
        composition.key_down(&KeyB);
        assert_eq!(composition.display_string(), "ab");

        let composition = skk.start_composition();
        composition.key_down(&Shift);
        composition.key_down(&KeyA);
        composition.key_up(&Shift);
        composition.key_down(&KeyB);
        assert_eq!(composition.display_string(), "Ab");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyA);
        assert_eq!(composition.display_string(), "あ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyA);
        composition.key_down(&KeyI);
        assert_eq!(composition.display_string(), "あい");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyK);
        composition.key_down(&KeyA);
        assert_eq!(composition.display_string(), "か");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyT);
        composition.key_down(&KeyS);
        assert_eq!(composition.display_string(), "ts");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyT);
        composition.key_down(&KeyS);
        composition.key_down(&KeyU);
        assert_eq!(composition.display_string(), "つ");

        let composition = skk.start_composition_as(TransformerTypes::Direct);
        composition.key_down(&KeyA);
        composition.key_up(&KeyA);
        composition.key_down(&Ctrl);
        composition.key_down(&KeyJ);
        composition.key_up(&Ctrl);
        composition.key_up(&KeyJ);
        composition.key_down(&KeyA);
        composition.key_up(&KeyA);
        composition.key_down(&KeyL);
        composition.key_up(&KeyL);
        composition.key_down(&KeyA);
        composition.key_up(&KeyA);
        assert_eq!(composition.display_string(), "aあa");
    }
}
