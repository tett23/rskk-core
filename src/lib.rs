mod composition;
mod composition_buffer;
mod keyboards;
mod transformers;

use composition::Composition;
use transformers::TransformerTypes;

pub struct RSKK {
    compositions: Vec<Composition>,
    default_composition_type: TransformerTypes,
    keyboard_type: keyboards::Keyboards,
}

impl RSKK {
    pub fn new(
        keyboard_type: keyboards::Keyboards,
        default_composition_type: TransformerTypes,
    ) -> Self {
        RSKK {
            compositions: vec![],
            default_composition_type,
            keyboard_type,
        }
    }

    pub fn start_composition(&mut self) -> &mut Composition {
        self.start_composition_as(self.default_composition_type)
    }

    pub fn start_composition_as(&mut self, composition_type: TransformerTypes) -> &mut Composition {
        self.compositions
            .push(Composition::new(&self.keyboard_type, composition_type));

        self.compositions.last_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keyboards::KeyCode;

    #[test]
    fn it_works() {
        let mut skk = RSKK::new(keyboards::Keyboards::US, TransformerTypes::Direct);
        let composition = skk.start_composition();
        composition.key_down(&KeyCode::KeyA);
        composition.key_down(&KeyCode::KeyB);
        assert_eq!(composition.display_string(), "ab");

        let composition = skk.start_composition();
        composition.key_down(&KeyCode::Shift);
        composition.key_down(&KeyCode::KeyA);
        composition.key_up(&KeyCode::Shift);
        composition.key_down(&KeyCode::KeyB);
        assert_eq!(composition.display_string(), "Ab");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyCode::KeyA);
        assert_eq!(composition.display_string(), "あ");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyCode::KeyA);
        composition.key_down(&KeyCode::KeyI);
        assert_eq!(composition.display_string(), "あい");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyCode::KeyK);
        composition.key_down(&KeyCode::KeyA);
        assert_eq!(composition.display_string(), "か");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyCode::KeyT);
        composition.key_down(&KeyCode::KeyS);
        assert_eq!(composition.display_string(), "ts");

        let composition = skk.start_composition_as(TransformerTypes::Hiragana);
        composition.key_down(&KeyCode::KeyT);
        composition.key_down(&KeyCode::KeyS);
        composition.key_down(&KeyCode::KeyU);
        assert_eq!(composition.display_string(), "つ");
    }
}
