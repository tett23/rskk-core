#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;

mod composition;
mod dictionary;
mod keyboards;
mod rskk_config;
mod tests;
mod transformers;

use std::convert::TryFrom;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::sync::{Arc, Mutex};

use composition::Composition;
use keyboards::KeyEvents;
use transformers::{Config, TransformerTypes};

pub use dictionary::{Dictionary, DictionaryEntry};
pub use rskk_config::{KeyConfig, RSKKConfig};

pub struct RSKK {
    config: Arc<RSKKConfig>,
    dictionary: Arc<Dictionary>,
    compositions: Vec<Composition>,
    default_composition_type: TransformerTypes,
}

impl RSKK {
    pub fn new(default_composition_type: TransformerTypes) -> Self {
        RSKK {
            config: Arc::new(RSKKConfig::default_config()),
            dictionary: Arc::new(Dictionary::new(set![])),
            compositions: vec![],
            default_composition_type,
        }
    }

    pub fn last_composition(&self) -> Option<&Composition> {
        self.compositions.last()
    }

    pub fn last_mut_composition(&mut self) -> Option<&mut Composition> {
        self.compositions.last_mut()
    }

    pub fn parse_dictionary(&mut self, dic: &str) {
        self.dictionary = Arc::new(Dictionary::parse(dic));
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

lazy_static! {
    static ref RSKK_INSTANCE: Arc<Mutex<RSKK>> =
        Arc::new(Mutex::new(RSKK::new(TransformerTypes::Direct)));
}

#[no_mangle]
pub extern "C" fn rskk_start_composition() -> c_int {
    (*RSKK_INSTANCE)
        .lock()
        .as_mut()
        .map(|rskk| {
            rskk.start_composition_as(TransformerTypes::Direct);
            1
        })
        .unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn rskk_push_key_event(event_type: u16, code: u16) -> bool {
    let event = KeyEvents::try_from((event_type, code));
    if event.is_err() {
        return false;
    }
    let event = event.unwrap();

    (*RSKK_INSTANCE)
        .lock()
        .as_mut()
        .map(|rskk| {
            rskk.last_mut_composition()
                .map(|composition| composition.push_key_event(&event))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn rskk_buffer_content() -> *mut c_char {
    let buf = (*RSKK_INSTANCE)
        .lock()
        .map(|rskk| {
            rskk.last_composition()
                .map(|composition| composition.buffer_content())
                .unwrap_or("".to_owned())
        })
        .unwrap_or("".to_owned());

    CString::new(buf).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rskk_display_string() -> *mut c_char {
    let buf = (*RSKK_INSTANCE)
        .lock()
        .map(|rskk| {
            rskk.last_composition()
                .map(|composition| composition.display_string())
                .unwrap_or("".to_owned())
        })
        .unwrap_or("".to_owned());

    CString::new(buf).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rskk_is_stopped() -> bool {
    (*RSKK_INSTANCE)
        .lock()
        .map(|rskk| {
            rskk.last_composition()
                .map(|composition| composition.is_stopped())
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn rskk_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn rskk_next_composition() -> c_int {
    (*RSKK_INSTANCE)
        .lock()
        .as_mut()
        .map(|rskk| {
            let tf = rskk
                .last_composition()
                .map({ |c| c.base_transformer_type() })
                .unwrap_or(TransformerTypes::Direct);
            rskk.start_composition_as(tf);
            1
        })
        .unwrap_or(-1)
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
            "ctrl" | "left_control" | "right_control" => {
                crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Ctrl)
            }
            "shift" | "left_shift" | "right_shift" => {
                crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Shift)
            }
            "alt" | "left_option" | "right_option" => {
                crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Alt)
            }
            "super" | "left_command" | "right_command" => {
                crate::keyboards::KeyCode::Meta(crate::keyboards::MetaKey::Super)
            }
            "enter" | "return" | "\n" => {
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
    ( $conf:expr, $t:expr ) => {{
        dbg!($t);
        let ret: Box<dyn crate::transformers::Transformable> = match $t {
            crate::transformers::TransformerTypes::Stopped(reason) => Box::new(
                crate::transformers::StoppedTransformer::new($conf, reason, ""),
            ),
            crate::transformers::TransformerTypes::Direct => {
                Box::new(crate::transformers::DirectTransformer::new($conf))
            }
            crate::transformers::TransformerTypes::Continuous => {
                Box::new(crate::transformers::ContinuousTransformer::new(
                    $conf,
                    crate::transformers::TransformerTypes::Hiragana,
                ))
            }
            crate::transformers::TransformerTypes::Hiragana => {
                Box::new(crate::transformers::HiraganaTransformer::new($conf))
            }
            crate::transformers::TransformerTypes::Henkan => {
                Box::new(crate::transformers::HenkanTransformer::new(
                    $conf,
                    crate::transformers::TransformerTypes::Hiragana,
                ))
            }
            _ => unreachable!(),
        };

        ret
    }};
    ( $conf:expr, ContinuousTransformer, $v:expr ) => {
        Box::new(crate::transformers::ContinuousTransformer::new($conf, $v))
    };
    ( $conf:expr, UnknownWordTransformer, $v:expr ) => {
        Box::new(crate::transformers::UnknownWordTransformer::new($conf, $v))
    };
    ( $conf:expr, HenkanTransformer, $v:expr ) => {
        Box::new(crate::transformers::HenkanTransformer::new($conf, $v))
    };
    ( $conf:expr, YomiTransformer, $v:expr ) => {
        Box::new(crate::transformers::YomiTransformer::new($conf, $v))
    };
}

#[macro_export]
macro_rules! tfe {
    ( $conf:expr, $t:tt; $input:expr ) => {{
        let tf = crate::tf!($conf.clone(), $t);
        let mut composition = crate::Composition::new_from_transformer(tf.config(), tf);
        let vec = crate::tests::str_to_key_code_vector($input);
        composition.push_key_events(&vec);

        composition.transformer()
    }};
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
    use crate::transformers::StoppedReason::*;
    use TransformerTypes::*;

    #[test]
    fn it_works() {
        let conf = dummy_conf();

        let items = tds![conf, Direct;
            ["a", "a", Stopped(Compleated)],
            ["A", "A", Stopped(Compleated)],
            ["[down:ctrl]j[up:ctrl]a", "あ", Stopped(Compleated)],
            ["[down:ctrl]j[up:ctrl]Henkann", "▽へんかん", Henkan],
            ["[down:ctrl]j[up:ctrl]Henkann[backspace]", "▽へんか", Henkan],
            ["[down:ctrl]j[up:ctrl]Henkann[backspace]nn", "▽へんかん", Henkan],
        ];
        test_transformer(items);
    }
}
