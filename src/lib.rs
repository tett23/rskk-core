mod composition;
mod dictionary;
mod keyboards;
mod rskk_config;
mod tests;
mod transformers;

use composition::Composition;
use std::rc::Rc;
use transformers::{Config, TransformerTypes};

pub use dictionary::{Dictionary, DictionaryEntry};
pub use rskk_config::{KeyConfig, RSKKConfig};

pub struct RSKK {
    config: Rc<RSKKConfig>,
    dictionary: Rc<Dictionary>,
    compositions: Vec<Composition>,
    default_composition_type: TransformerTypes,
}

impl RSKK {
    pub fn new(default_composition_type: TransformerTypes) -> Self {
        RSKK {
            config: Rc::new(RSKKConfig::default_config()),
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
            Config::new(self.config.clone(), self.dictionary.clone()),
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
          let mut temp_set = std::collections::HashSet::new();
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
          let mut temp_set = std::collections::HashSet::new();
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
          let mut temp_set = std::collections::HashSet::new();
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
            "enter" | "\n" => {
                crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Enter, '\n')
            }
            "space" | " " => {
                crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Space, ' ')
            }
            "tab" | "\t" => {
                crate::keyboards::KeyCode::PrintableMeta(crate::keyboards::MetaKey::Tab, '\t')
            }
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

#[macro_export]
macro_rules! tf {
    ( $conf:expr, $t:expr ) => {
        match $t {
            transformers::TransformerTypes::Direct => {
                Box::new(crate::transformers::DirectTransformer::new($conf))
            }
            transformers::TransformerTypes::Hiragana => {
                Box::new(crate::transformers::HiraganaTransformer::new($conf))
            }
            _ => unreachable!(),
        }
    };
    ( $conf:expr, ContinuousTransformer, $v:expr  ) => {
        Box::new(crate::transformers::ContinuousTransformer::new($conf, $v))
    };
    ( $conf:expr, UnknownWordTransformer, $v:expr ) => {
        Box::new(crate::transformers::UnknownWord::new($conf, $v))
    };
}

#[macro_export]
macro_rules! td {
    ($conf:expr, $tf:tt; [$input:expr, $out:expr, $out_tf:expr]) => {{
        crate::tests::TestData::new(crate::tf!($conf, $tf), $input, $out, $out_tf)
    }};
    ($conf:expr, $tf:tt, $tf_v1:expr; [$input:expr, $out:expr, $out_tf:expr]) => {{
        crate::tests::TestData::new(crate::tf!($conf, $tf, $tf_v1), $input, $out, $out_tf)
    }};
}

#[macro_export]
macro_rules! tds {
    ( $conf:expr, $tf:tt; $( [ $($x:expr),* $(,)? ] ),* $(,)? ) => {{
        vec![
            $( crate::td![$conf.clone(), $tf; [ $($x),* ]], )*
        ]
    }};
    ( $conf:expr, $tf:tt, $tf_v1:expr; $( [ $($x:expr),* $(,)? ] ),* $(,)? ) => {{
        vec![
            $( crate::td![$conf.clone(), $tf, $tf_v1; [ $($x),* ] ], )*
        ]
    }};
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::tests::{dummy_conf, test_transformer};
    use transformers::Word;
    use TransformerTypes::*;

    #[test]
    fn it_works() {
        let conf = dummy_conf();

        let items = tds![conf, Direct;
            ["a", "a", Stopped],
            ["A", "A", Stopped],
            ["[down:ctrl]j[up:ctrl]a", "あ", Stopped]
        ];
        test_transformer(items);

        let items = tds![conf, Hiragana;
            ["a", "あ", Stopped],
            ["ka", "か", Stopped],
            ["ts", "ts", Hiragana],
            ["tsu", "つ", Stopped],
            ["K", "▽k", Henkan],
            ["Ka", "▽か", Henkan],
            ["Kannji", "▽かんじ", Henkan],
            ["Kannji ", "▼漢字", Henkan],
            ["Kannji \n", "漢字", Stopped],
            ["Michigo ", "[登録: みちご]", Henkan],
            ["Michigo \n", "", Stopped]
        ];
        test_transformer(items);

        let items = tds![conf, ContinuousTransformer, Hiragana;
            ["hiragana", "ひらがな", ContinuousTransformer],
            ["hiragana\n", "ひらがな", Stopped],
            ["hiragana[escape]", "", Canceled]
        ];
        test_transformer(items);

        let items = tds![conf, UnknownWordTransformer, Word::new("みちご", None);
            ["hiragana", "[登録: みちご]ひらがな", UnknownWord],
            ["Kannji", "[登録: みちご]▽かんじ", UnknownWord],
            ["Kannji ", "[登録: みちご]▼漢字", UnknownWord],
            // ["Kannji \n","[登録: みちご]漢字", UnknownWord],
            // ["Michi \nGo \n\n","未知語",Stopped]
        ];
        test_transformer(items);
    }
}
