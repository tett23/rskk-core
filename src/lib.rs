#![feature(box_syntax)]
#![feature(slice_patterns)]
#![allow(improper_ctypes)]

#[macro_use]
extern crate serde;
extern crate kana;

mod composition;
mod dictionary;
mod keyboards;
mod rskk_config;
mod tests;
mod transformers;

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::rc::Rc;

use composition::Composition;
use keyboards::KeyEvents;
use transformers::{Config, TransformerTypes};

pub use dictionary::{Dictionary, DictionaryEntry};
pub use rskk_config::{KeyConfig, RSKKConfig};

pub struct RSKK {
    config: Rc<RSKKConfig>,
    dictionary: Rc<Dictionary>,
    default_composition_type: TransformerTypes,
}

impl RSKK {
    pub fn new(default_composition_type: TransformerTypes) -> Self {
        RSKK {
            config: Rc::new(RSKKConfig::default_config()),
            dictionary: Rc::new(Dictionary::new(set![])),
            default_composition_type,
        }
    }

    pub fn parse_dictionary(&mut self, dic: &str) {
        self.dictionary = Rc::new(Dictionary::parse(dic));
    }

    pub fn parse_config(&mut self, config_json: &str) -> Result<(), &str> {
        serde_json::from_str(config_json)
            .map(|config| self.config = Rc::new(config))
            .map(|_| ())
            .or({ Err("") })
    }

    pub fn start_composition(&self) -> Composition {
        self.start_composition_as(self.default_composition_type)
    }

    pub fn start_composition_as(&self, composition_type: TransformerTypes) -> Composition {
        Composition::new(
            Config::new(self.config.clone(), self.dictionary.clone()),
            composition_type,
        )
    }
}

#[no_mangle]
pub extern "C" fn rskk_new() -> *mut RSKK {
    Box::into_raw(box RSKK::new(TransformerTypes::Direct))
}

#[no_mangle]
pub extern "C" fn rskk_parse_config_json(rskk: *mut RSKK, json: *const c_char) -> bool {
    match (unsafe { rskk.as_mut() }, unsafe {
        CStr::from_ptr(json).to_str()
    }) {
        (Some(rskk), Ok(json)) => Ok((rskk, json)),
        _ => Err(""),
    }
    .map(|(rskk, json)| rskk.parse_config(json))
    .map_or_else(|_| false, |_| true)
}

#[no_mangle]
pub extern "C" fn rskk_parse_dictionary(rskk: *mut RSKK, dic: *const c_char) -> bool {
    match (unsafe { rskk.as_mut() }, unsafe {
        CStr::from_ptr(dic).to_str()
    }) {
        (Some(rskk), Ok(dic)) => Ok((rskk, dic)),
        _ => Err(""),
    }
    .map(|(rskk, dic)| rskk.parse_dictionary(dic))
    .map_or_else(|_| false, |_| true)
}

#[no_mangle]
pub extern "C" fn rskk_free_rskk(raw: *mut RSKK) {
    unsafe { Box::from_raw(raw) };
}

#[no_mangle]
pub extern "C" fn rskk_start_composition(rskk: *mut RSKK) -> *mut Composition {
    unsafe { rskk.as_ref() }
        .map(|rskk| Box::into_raw(box rskk.start_composition_as(TransformerTypes::Direct)))
        .unwrap()
}

#[no_mangle]
pub extern "C" fn rskk_free_composition(raw_composition: *mut Composition) {
    unsafe { Box::from_raw(raw_composition) };
}

#[no_mangle]
pub extern "C" fn rskk_next_composition(
    rskk: *mut RSKK,
    composition: *mut Composition,
) -> *mut Composition {
    unsafe { rskk.as_ref() }
        .map(|rskk| {
            let tf = unsafe {
                composition
                    .as_ref()
                    .map(|c| c.base_transformer_type())
                    .unwrap_or(TransformerTypes::Direct)
            };
            Box::into_raw(box rskk.start_composition_as(tf))
        })
        .unwrap()
}

#[no_mangle]
pub extern "C" fn rskk_push_key_event(
    composition: *mut Composition,
    event_type: u16,
    code: u16,
) -> bool {
    let event = KeyEvents::try_from((event_type, code));
    if event.is_err() {
        return false;
    }
    let event = event.unwrap();

    unsafe { composition.as_mut() }
        .map({ |c| c.push_key_event(&event) })
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn rskk_buffer_content(composition: *mut Composition) -> *mut c_char {
    let buf = unsafe { composition.as_ref() }
        .map(|c| c.buffer_content())
        .unwrap_or("".to_owned());

    CString::new(buf).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rskk_display_string(composition: *mut Composition) -> *mut c_char {
    let buf = unsafe { composition.as_ref() }
        .map(|c| c.display_string())
        .unwrap_or("".to_owned());

    CString::new(buf).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rskk_is_stopped(composition: *mut Composition) -> bool {
    unsafe { composition.as_ref().map(|c| c.is_stopped()).unwrap_or(true) }
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
            crate::transformers::TransformerTypes::Katakana => {
                Box::new(crate::transformers::KatakanaTransformer::new($conf))
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
    ( $tf:expr ) => {
        box $tf
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
    ($tf:expr; [$input:expr, $out:expr, $out_tf:expr]) => {{
        crate::tests::TestData::new(crate::tf!($tf), $input, $out, $out_tf)
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
    ( $tf:expr; $( [ $($x:expr),* $(,)? ] ),* $(,)? ) => {{
        vec![
            $( crate::td![$tf.clone(); [ $($x),* ] ], )*
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
            ["[down:ctrl]j[up:ctrl]Kanji \n", "漢字", Stopped(Compleated)],
            ["[down:ctrl]j[up:ctrl]qka", "カ", Stopped(Compleated)],
            ["[down:ctrl]j[up:ctrl]Katakanaq", "カタカナ", Stopped(Compleated)],
            ["[down:ctrl]j[up:ctrl]qHiraganaq", "ひらがな", Stopped(Compleated)],
        ];
        test_transformer(items);
    }
}
