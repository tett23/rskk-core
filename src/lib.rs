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

#[macro_export]
macro_rules! tf {
    ( $t:expr, $conf:expr ) => {
        match $t {
            transformers::TransformerTypes::Direct => {
                Box::new(transformers::DirectTransformer::new($conf))
            }
            transformers::TransformerTypes::Hiragana => {
                Box::new(transformers::HiraganaTransformer::new($conf))
            }
            _ => unreachable!(),
        }
    };
    ( ContinuousTransformer, $conf:expr, $v:expr  ) => {
        Box::new(transformers::ContinuousTransformer::new($conf, $v))
    };
    ( UnknownWordTransformer, $conf:expr, $v:expr ) => {
        Box::new(transformers::UnknownWord::new($conf, $v))
    };
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::tests::{dummy_conf, str_to_key_code_vector};
    use transformers::Transformer;
    use transformers::Word;
    use TransformerTypes::*;

    #[derive(Debug)]
    struct TestData<S: Into<String>>(Box<dyn Transformer>, S, S, TransformerTypes);

    #[test]
    fn it_works2() {
        let conf = dummy_conf();
        // conf.dictionary = Dictionary::parse(
        //     "かんじ/漢字/
        //     みち/未知/
        //     ご/語/",
        // );

        let items = vec![
            TestData(tf!(Direct, conf.clone()), "a", "a", Stopped),
            TestData(tf!(Direct, conf.clone()), "A", "A", Stopped),
            TestData(tf!(Hiragana, conf.clone()), "a", "あ", Stopped),
            TestData(tf!(Hiragana, conf.clone()), "ka", "か", Stopped),
            TestData(tf!(Hiragana, conf.clone()), "ts", "ts", Hiragana),
            TestData(tf!(Hiragana, conf.clone()), "tsu", "つ", Stopped),
            TestData(
                tf!(Direct, conf.clone()),
                "[down:ctrl]j[up:ctrl]a",
                "あ",
                Stopped,
            ),
            TestData(tf!(Hiragana, conf.clone()), "K", "▽k", Henkan),
            TestData(tf!(Hiragana, conf.clone()), "Ka", "▽か", Henkan),
            TestData(tf!(Hiragana, conf.clone()), "Kannji", "▽かんじ", Henkan),
            TestData(
                tf!(Hiragana, conf.clone()),
                "Kannji[space]",
                "▼漢字",
                Henkan,
            ),
            TestData(
                tf!(Hiragana, conf.clone()),
                "Kannji[space][enter]",
                "漢字",
                Stopped,
            ),
            TestData(
                tf!(ContinuousTransformer, conf.clone(), Hiragana),
                "hiragana",
                "ひらがな",
                ContinuousTransformer,
            ),
            TestData(
                tf!(ContinuousTransformer, conf.clone(), Hiragana),
                "hiragana[enter]",
                "ひらがな",
                Stopped,
            ),
            TestData(
                tf!(ContinuousTransformer, conf.clone(), Hiragana),
                "hiragana[escape]",
                "",
                Canceled,
            ),
            TestData(
                tf!(
                    UnknownWordTransformer,
                    conf.clone(),
                    Word::new("みちご", None)
                ),
                "hiragana",
                "[登録: みちご]ひらがな",
                UnknownWord,
            ),
            TestData(
                tf!(
                    UnknownWordTransformer,
                    conf.clone(),
                    Word::new("みちご", None)
                ),
                "Kannji",
                "[登録: みちご]▽かんじ",
                UnknownWord,
            ),
            TestData(
                tf!(
                    UnknownWordTransformer,
                    conf.clone(),
                    Word::new("みちご", None)
                ),
                "Kannji[space]",
                "[登録: みちご]▼漢字",
                UnknownWord,
            ),
            TestData(
                tf!(
                    UnknownWordTransformer,
                    conf.clone(),
                    Word::new("みちご", None)
                ),
                "Kannji[space][enter]",
                "[登録: みちご]漢字",
                Stopped,
            ),
            // TestData(
            //     tf!(
            //         UnknownWordTransformer,
            //         conf.clone(),
            //         Word::new("みちご", None)
            //     ),
            //     "Michigo[space]Michi[space]",
            //     "[登録: みちご]▼未知",
            //     Stopped,
            // ),
            TestData(
                tf!(
                    UnknownWordTransformer,
                    conf.clone(),
                    Word::new("みちご", None)
                ),
                "Michi[down:space][down:enter]Go[down:space][down:enter]",
                "未知語",
                Stopped,
            ),
            TestData(
                tf!(Hiragana, conf.clone()),
                "Michigo[space]",
                "[登録: みちご]",
                Henkan,
            ),
            TestData(
                tf!(Hiragana, conf.clone()),
                "Michigo[space][enter]",
                "",
                Canceled,
            ),
        ];

        items.into_iter().for_each(
            |TestData(start_transformer, input, output, out_transformer)| {
                let mut composition =
                    Composition::new_from_transformer(conf.clone(), start_transformer);
                composition.push_key_events(&str_to_key_code_vector(input));
                assert_eq!(
                    (out_transformer, output.into()),
                    (composition.transformer_type(), composition.display_string()),
                    "{}",
                    input
                );
            },
        );
    }

    // #[test]
    // fn it_works() {
    //     let mut skk = RSKK::new(TransformerTypes::Direct);
    //     skk.parse_dictionary(
    //         "かんじ/漢字/
    //     みち/未知/
    //     ご/語/",
    //     );

    //     let items = vec![TestData(tf!(Direct, conf), "a", "a", Stopped)];

    //     items.into_iter().for_each(
    //         |TestData(start_transformer, input, output, out_transformer)| {
    //             let composition = skk.start_composition_as(start_transformer);
    //             composition.push_key_events(&str_to_key_code_vector(input));
    //             assert_eq!(
    //                 (out_transformer, output.into()),
    //                 (composition.transformer_type(), composition.display_string()),
    //                 "{:?}",
    //                 TestData(start_transformer, input, output, out_transformer)
    //             );
    //         },
    //     );
    // }
}
