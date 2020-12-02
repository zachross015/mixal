use crate::word::Word;

pub struct MagneticTapeUnit {
    unit_number: u8,
    block: [Word; 100],
}

impl MagneticTapeUnit {
    pub fn new(number: u8, contents: [Word; 100]) -> MagneticTapeUnit {
        MagneticTapeUnit {
            unit_number: number,
            block: contents,
        }
    }
}