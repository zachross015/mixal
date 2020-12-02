use std::fmt;
use crate::instruction::adjusted_field_specification;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Word {
    pub positive: bool,
    pub bytes: [u8; 5],
}

impl Word {
    pub fn new(positive: bool, b: [u8; 5]) -> Word {
        Word {
            positive: positive,
            bytes: b,
        }
    }

    pub fn default() -> Word {
        Word::new(true, [0; 5])
    }

    pub fn from_value(value: i64) -> Word {
        let positive = value < 0;
        let mut bytes : [u8; 5] = [0; 5];
        let mut value_mut = value.clone();
        for i in 0..5 {
            bytes[4 - i] = (value_mut % 256) as u8;
            value_mut = value_mut >> 8;
        }
        Word::new(positive, bytes)
    }

    pub fn address(&self) -> usize {
        self.field_value((1, 2)) as usize
    }

    pub fn index(&self) -> u8 {
        self.bytes[2]
    }

    // 8L + R in (L:R)
    pub fn field(&self) -> u8 {
        self.bytes[3]
    }

    pub fn opcode(&self) -> u8 {
        self.bytes[4]
    }

    pub fn negate(&self) -> Word {
        let mut new_word = self.clone();
        new_word.positive = !new_word.positive;
        new_word
    }

    pub fn field_value(&self, field_specification: (usize, usize)) -> i64 {
        let (zero_included, only_zero, (l, r)) = adjusted_field_specification(field_specification);
        if only_zero { return 0; }
    
        let mut result = self.bytes[l] as i64;
        for i in (l + 1)..=(r) {
            result = result << 8;
            result = result + (self.bytes[i] as i64);
        }
        result * (if zero_included && !self.positive { -1 } else { 1 })
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2} {:>4} {:>4} {:>4} {:>4} {:>4}", 
            if self.positive { '+' } else { '-' },
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4]
        )
    }
}
