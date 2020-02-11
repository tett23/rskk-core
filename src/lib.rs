#![feature(box_syntax)]
#![feature(slice_patterns)]
#![feature(type_name_of_val)]
#![allow(improper_ctypes)]

#[macro_use]
extern crate serde;
extern crate kana;

mod composition;
mod composition_result;
mod context;
mod dictionary;
mod keyboards;
mod rskk_config;
mod tests;
mod transformers;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::rc::Rc;

use composition::Composition;
use keyboards::KeyEvents;
use transformers::TransformerTypes;

pub use composition_result::CompositionResult;
pub use context::{Context, Contexts};
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
            Rc::new(RefCell::new(Context::new(
                self.config.clone(),
                self.dictionary.clone(),
            ))),
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
pub extern "C" fn rskk_stopped_buffer(composition: *mut Composition) -> *mut c_char {
    let buf = unsafe { composition.as_ref() }
        .map(|c| c.stopped_buffer())
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
macro_rules! tf {
    ( $conf:expr, $t:expr ) => {{
        let conf = $conf.clone();
        let ret: Box<dyn crate::transformers::Transformable> = match $t {
            crate::transformers::TransformerTypes::Stopped(reason) => {
                Box::new(crate::transformers::StoppedTransformer::new(conf, reason))
            }
            crate::transformers::TransformerTypes::Direct => {
                Box::new(crate::transformers::DirectTransformer::new(conf))
            }
            crate::transformers::TransformerTypes::Continuous => {
                Box::new(crate::transformers::ContinuousTransformer::new(
                    conf,
                    crate::transformers::TransformerTypes::Hiragana,
                ))
            }
            crate::transformers::TransformerTypes::Hiragana => {
                Box::new(crate::transformers::HiraganaTransformer::new(conf))
            }
            crate::transformers::TransformerTypes::Katakana => {
                Box::new(crate::transformers::KatakanaTransformer::new(conf))
            }
            crate::transformers::TransformerTypes::Abbr => {
                Box::new(crate::transformers::AbbrTransformer::new(conf))
            }
            crate::transformers::TransformerTypes::Henkan => {
                Box::new(crate::transformers::HenkanTransformer::new(
                    conf,
                    crate::transformers::TransformerTypes::Hiragana,
                ))
            }
            _ => unreachable!(),
        };

        ret
    }};
    ( $conf:expr, ContinuousTransformer, $v:expr ) => {
        Box::new(crate::transformers::ContinuousTransformer::new(
            $conf.clone(),
            $v,
        ))
    };
    ( $conf:expr, UnknownWordTransformer, $v:expr ) => {
        Box::new(crate::transformers::UnknownWordTransformer::new(
            $conf.clone(),
            $v,
        ))
    };
    ( $conf:expr, HenkanTransformer, $v:expr ) => {
        Box::new(crate::transformers::HenkanTransformer::new(
            $conf.clone(),
            $v,
        ))
    };
    ( $conf:expr, YomiTransformer, $v:expr ) => {
        Box::new(crate::transformers::YomiTransformer::new($conf.clone(), $v))
    };
    ( $tf:expr ) => {
        box $tf
    };
}

#[macro_export]
macro_rules! tfe {
    ( $conf:expr, $t:tt; $input:expr ) => {{
        let tf = crate::tf!($conf.clone(), $t);
        let mut composition = crate::Composition::new_from_transformer(tf.clone_context(), tf);
        let vec = crate::tests::str_to_key_code_vector($input);
        composition.push_key_events(&vec);

        composition.transformer()
    }};
}

#[macro_export]
macro_rules! tds {
    (
        $( $tf_args:tt ),* $(,)? ;
        $([
            $input:expr, { $( $td_key:tt : $td_value:expr ),* $(,)? } $(,)?
        ]),* $(,)?
    ) => {{
        let tf = crate::tf!( $( $tf_args ),* );

        vec![
            $(
                crate::tests::helpers::TestData::new($input, tf.clone(), crate::td!({ $( $td_key: $td_value ),* } ))
            ),*
        ]
    }}
}

#[macro_export]
macro_rules! td {
  ({
    $(
      $key:tt : $value:expr
    ),* $(,)?
  }) => {{
    let mut ret = crate::tests::helpers::Example::new();
    $(
        ret.$key($value);
    )*

    ret
  }};
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::tests::dummy_context;
    use crate::transformers::StoppedReason::*;
    use TransformerTypes::*;

    #[test]
    fn it_works() {
        let conf = dummy_context();

        let vec = tds![conf, Direct;
            ["a", { display: "", stopped_buffer: "a", transformer_type: Stopped(Compleated) }],
            ["A", { display: "", stopped_buffer: "A", transformer_type: Stopped(Compleated) }],
            ["[down:ctrl]j[up:ctrl]a", { display: "", stopped_buffer: "あ", transformer_type: Stopped(Compleated) }],
            ["[down:ctrl]j[up:ctrl]Henkann", { display: "▽へんかん", transformer_type: Henkan }],
            ["[down:ctrl]j[up:ctrl]Henkann[backspace]", { display: "▽へんか", transformer_type: Henkan }],
            ["[down:ctrl]j[up:ctrl]Henkann[backspace]nn", { display: "▽へんかん", transformer_type: Henkan }],
            ["[down:ctrl]j[up:ctrl]Kanji \n", { display: "", stopped_buffer: "漢字", transformer_type: Stopped(Compleated) }],

            ["[down:ctrl]j[up:ctrl]qka", { display: "", stopped_buffer: "カ", transformer_type: Stopped(Compleated) }],
            ["[down:ctrl]j[up:ctrl]Katakanaq", { display: "", stopped_buffer: "カタカナ", transformer_type: Stopped(Compleated) }],
            ["[down:ctrl]j[up:ctrl]qHiraganaq", { display: "", stopped_buffer: "ひらがな", transformer_type: Stopped(Compleated) }],
        ];
        crate::tests::helpers::TestData::batch(vec);
    }
}
