use crate::word::Word;

pub struct DiskDrumUnit {
    unit_number: u8,
    block: [Word; 100],
}

impl DiskDrumUnit {
    pub fn new(number: u8, contents: [Word; 100]) -> DiskDrumUnit {
        DiskDrumUnit {
            unit_number: number,
            block: contents,
        }
    }
}