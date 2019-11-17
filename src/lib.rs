mod composition;
mod composition_buffer;
mod composition_types;
mod keyboards;
mod keycodes;
mod transformers;

use composition::Composition;
use composition_types::CompositionType;

pub struct RSKK {
    compositions: Vec<Composition>,
    default_composition_type: CompositionType,
    keyboard_type: keyboards::Keyboards,
}

impl RSKK {
    pub fn new(
        keyboard_type: keyboards::Keyboards,
        default_composition_type: CompositionType,
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

    pub fn start_composition_as(&mut self, composition_type: CompositionType) -> &mut Composition {
        self.compositions
            .push(Composition::new(&self.keyboard_type, composition_type));

        self.compositions.last_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keycodes::KeyCode;

    #[test]
    fn it_works() {
        let mut skk = RSKK::new(keyboards::Keyboards::US, CompositionType::Direct);
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
    }
}
