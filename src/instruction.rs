use crate::computer::Computer;
use crate::word::{Word};

/// Provides a useful macro for checking conditions involving adjusted field 
/// specifications. 
/// 
/// The `word_zero_condition` checks first if the value zero is included in 
/// the field specification. If it is, the sign of the receiving word is adjusted 
/// to match that of the sending word. After that, the macro checks if 
/// zero is the *only* part of the adjusted field specification, in which 
/// the function is told to terminate since there is no further action to be 
/// taken.
/// 
/// ## Arguments
/// 
/// - `zero_included`: Boolean of whether or not the value 0 is included in the 
/// adjusted field specification. 
/// - `only_zero`: Boolean of whether or not the value 0 is the only value in 
/// the adjusted field specification. 
/// - `from_word`: The sending word in the comparison. This is used to get the 
/// `sign` of the sending word to adjust the receiver if necessary. 
/// - `to_word`: The receiving word in the comparison. This is necessary 
/// so its `sign` value can be adjusted if zero is part of the adjusted field 
/// specification.
/// 
macro_rules! word_zero_condition {
    ($z:ident, $o:ident, $f:ident, $t:ident) => {
        if $z {
            $t.sign = $f.sign;
            if $o {
                return;
            }
        }
    }
}

/// Provides a useful macro for creating instructions, so that the amount 
/// of copy and paste code is minimized. 
/// 
/// ## Arguments
/// - Instruction Name: the name of the instruction being created. This is usually the 
/// verbatim word used in MIX.
/// - *Optional* `parameter: type`: There is an optional list of paramters to be 
/// used in each instruction definition. For this, just input the standard rust 
/// definition of `parameter: type` pairings and they will be generated in the instruction 
/// struct. 
/// - `(self, computer) { ... }`: This is a mandatory block of code necessary 
/// to make the instruction run. This block of code is macro for the `execute_on`
/// implementation of the instruction for this specific instruction. The `(self, computer)` 
/// is necessary before the block since these variables need to be included in 
/// the function definition and macro expansions don't allow them to just be entered 
/// in the macro by default.
macro_rules! create_instruction {
    ($i:ident, ($s:ident, $c:ident) $body:block) => {
        pub struct $i {}
        impl $i {
            pub fn new() -> $i { $i {} }
        }
        impl Instruction for $i {
            fn execute_on(&$s, $c: &mut Computer) {
                $body
            }
        }
    };
    ($i:ident, $($v:ident: $t:ty),*, ($s:ident, $c:ident) $body:block) => {
        pub struct $i {
            $(pub $v: $t),*
        }
        impl $i {
            pub fn new($($v: $t),*) -> $i {
                $i {
                    $($v: $v),*
                }
            }
        }
        impl Instruction for $i {
            fn execute_on(&$s, $c: &mut Computer) {
                $body
            }
        }
    };
}

/// Provides a useful conversion for the field specification of a MIX command. 
/// 
/// In the documentation for MIX, a word is laid out from left to right as the 
/// indices 0-6, with the 0-byte indicating the sign. In the program implementation,
/// we instead have the sign byte as just a boolean value, while the rest of the 
/// bytes are the 0-5 indices of the word. This function takes the given 
/// field specification and gives us the tuple informing us of critical properties.
/// 
/// ## Arguments
/// 
/// - `field_specification`: An un-adjusted field specification, i.e. a field spec 
/// not previously run through this function.
/// 
/// ## Returns 
/// 
/// - `(zero_is_included, only_zero, (left_adjusted_index, right_adjusted_index))`. 
pub fn adjusted_field_specification(field_specification: (usize, usize)) -> (bool, bool, (usize, usize)) {
    let (left, right) = field_specification;
    let (l, r) = (left.max(1) - 1, right.max(1) - 1);
    (left == 0, right == 0, (l, r))
}

/// Copies the individual bytes from one word to another, given their field specification. 
/// 
/// ## Arguments
/// - `from_word`: A reference to the sending word. 
/// - `to_word`: A mutable reference to the receiving word.
/// - `field_specification`: An un-adjusted field specification for which fields should be copied.
pub fn copy_word_fields(from_word: &Word, to_word: &mut Word, field_specification: (usize, usize)) {
    let (zero_included, only_zero, (l, r)) = adjusted_field_specification(field_specification);
    word_zero_condition!(zero_included, only_zero, from_word, to_word);
    for i in (l)..=(r) {
        to_word.bytes[i] = from_word.bytes[i];
    }
}

/// Copies the individual bytes from one word to another, given their field specification. 
/// As per the description of index registers, the function fails if the field
/// specification does not contain valid index register values (0, 4, or 5).
/// 
/// ## Arguments
/// - `from_word`: A reference to the sending word. 
/// - `to_word`: A mutable reference to the receiving word.
/// - `field_specification`: An un-adjusted field specification for which fields should be copied.
/// 
/// ## Panics 
/// Panics whenever the field specification does not contain either 0, 4, or 5, since 
/// those are necessary for the index registers.
pub fn copy_word_fields_i(from_word: &Word, to_word: &mut Word, field_specification: (usize, usize)) {
    let (zero_included, only_zero, (l, r)) = adjusted_field_specification(field_specification);
    if (0..=2).contains(&l) && (0..=2).contains(&r) && !zero_included {
        panic!("[Error copy_word_fields_i] Invalid field specification given for index. Must include either 0, 4, or 5 (Given: {:?}).", field_specification);
    }
    word_zero_condition!(zero_included, only_zero, from_word, to_word);
    for i in (l.max(3))..=(r.min(4)) {
        to_word.bytes[i] = from_word.bytes[i];
    }
}

/// Stores the individual bytes from one register to a word, given their field specification. 
/// 
/// This is meant for the `store` instruction rather than `load`, since the 
/// field specification value for `store` specifies that the operation 
/// places the `r - l` upper bytes of the register into indices `(l,r)` of the 
/// receiving memory. 
/// 
/// ## Arguments
/// - `from_word`: A reference to the storing register. 
/// - `to_word`: A mutable reference to the receiving word of memory.
/// - `field_specification`: An un-adjusted field specification for which fields should be copied.
pub fn store_operation(from_word: &Word, to_word: &mut Word,  field_specification: (usize, usize)) {
    let (zero_included, only_zero, (l, r)) = adjusted_field_specification(field_specification);
    word_zero_condition!(zero_included, only_zero, from_word, to_word);

    let offset_l = 4 - (r - l);
    for i in 0..=(r-l) {
        to_word.bytes[l + i] = from_word.bytes[offset_l + i];
    } 
}

/// Matches the `index` to the corresponding index register, and returns a mutable 
/// reference to that register.
/// 
/// ## TODO
/// - Make the return value a `Result<&mut Word, ErrorThing>` so that error handling 
/// can be better managed by other functions. This will need a refactor but it 
/// should make debugging simpler once we have actual MIXAL code.
/// 
/// ## Arguments
/// - `computer`: A mutable reference to the computer we are retrieving the index 
/// from.
/// - `index`: The number corresponding to the index register that we are using.
/// It must be in the range 1-6.
/// 
/// ## Returns 
/// - A mutable reference to the corresponding index register, if it is found. Panics otherwise.
/// 
/// ## Panics
/// Panics when the index given in the argument is not in the range 1-6.
pub fn register_for_index(computer: &mut Computer, index: u8) -> &mut Word {
    let word = match index {
        1 => &mut computer.ri1,
        2 => &mut computer.ri2,
        3 => &mut computer.ri3,
        4 => &mut computer.ri4,
        5 => &mut computer.ri5,
        6 => &mut computer.ri6,
        _ => {
            // Throw error 
            panic!("[Error register_for_index] Invalid index given for decode. Must be in the range 1-6 (Given {}).", index);
        }
    };
    word
}

/// Adds two words 
pub fn add_words(word1: &Word, word2: &Word, field_specification: (usize, usize)) -> (Word, bool) {
    let word1_value = word1.field_value(field_specification).clone();
    let word2_value = word2.field_value(field_specification);
    let mut word = Word::default();
    let mut sum : i64 = word1_value + word2_value;

    let (zero_included, only_zero, (l, r)) = adjusted_field_specification(field_specification);
    if only_zero {
        panic!("[Error add_words] Can't add two numbers solely by their sign. (Input given {:#?})", field_specification);
    }

    if zero_included {
        word.sign = if sum >= 0 { true } else { false };
    }

    sum = sum.abs();
    for i in (l..=r).rev() {
        word.bytes[i] = (sum % 256) as u8;
        sum = sum >> 8;
    }

    if sum != 0 {
        return (word, true);
    }

    (word, false)
}

pub fn multiply_words(word1: &Word, word2: &Word, field_specification: (usize, usize)) -> (Word, Word) {
    let word1_value = word1.field_value((0,5)).clone();
    let word2_value = word2.field_value(field_specification);
    let mut word_lower = Word::default();
    let mut word_upper = Word::default();
    let mut product : i128 = (word1_value as i128) * (word2_value as i128);

    let (zero_included, only_zero, _) = adjusted_field_specification(field_specification);
    if only_zero {
        panic!("[Error multiply_words] Can't multiply two numbers solely by their sign. (Input given {:#?})", field_specification);
    }

    if zero_included {
        word_lower.sign = if product >= 0 { true } else { false };
        word_upper.sign = word_lower.sign;
    }

    product = product.abs();
    for i in (0..=4).rev() {
        word_lower.bytes[i] = (product % 256) as u8;
        product = product >> 8;
    }
    for i in (0..=4).rev() {
        word_upper.bytes[i] = (product % 256) as u8;
        product = product >> 8;
    }

    if product != 0 {
        return (word_upper, word_lower);
    }

    (word_upper, word_lower)
}

/// TODO: Test this.
pub fn divide_words(word1: &Word, word2: &Word, word3: &Word, field_specification: (usize, usize)) -> (Word, Word, bool) {
    let mut word_rem = Word::default();
    let mut word_div = Word::default();
    let divisor_value = word3.field_value(field_specification) as i128;
    if divisor_value == 0 {
        return (word_rem, word_div, true);
    }

    let word1_value = word1.field_value((1,5)).clone() as i128;
    let word_value = (word1_value << 40) | (word2.field_value((1,5)).clone() as i128);

    let mut dividend : i64 = ((word_value) / (divisor_value)) as i64;
    let mut remainder : i64 = ((word_value) % (divisor_value)) as i64;

    let (zero_included, only_zero, _) = adjusted_field_specification(field_specification);
    if only_zero {
        panic!("[Error divide_words] Can't divide two numbers solely by their sign. (Input given {:#?})", field_specification);
    }

    word_rem.sign = word1.sign;
    if zero_included {
        word_div.sign = word1.sign == word3.sign;
    }

    dividend = dividend;
    remainder = remainder;
    for i in (0..=4).rev() {
        word_rem.bytes[i] = (remainder % 256) as u8;
        word_div.bytes[i] = (dividend % 256) as u8;
        remainder = remainder >> 8;
        dividend = dividend >> 8;
    }

    (word_div, word_rem, false)
}

/// MARK: Instructions

pub trait Instruction {
    fn execute_on(&self, computer: &mut Computer);
}

create_instruction!(NoOperation, (self, _c) {});

create_instruction!(Halt, (self, computer) { computer.pc = 4000; });

create_instruction!(LoadA, address: usize, field_specification: (usize, usize), negative: bool, (self, computer) {
    let ra =  &mut computer.ra;
    let mem = &computer.memory[self.address];
    copy_word_fields(mem, ra, self.field_specification);
    if self.negative { ra.sign = !ra.sign; }
});

create_instruction!(LoadX, address: usize, field_specification: (usize, usize), negative: bool, (self, computer) {
    let rx =  &mut computer.rx;
    let mem = &computer.memory[self.address];
    copy_word_fields(mem, rx, self.field_specification);
    if self.negative { rx.sign = !rx.sign; }
});

create_instruction!(LoadI, index: u8, address: usize, field_specification: (usize, usize), negative: bool, (self, computer) {
    let mem = &computer.memory[self.address].clone();
    let ri =  register_for_index(computer, self.index);
    copy_word_fields_i(mem, ri, self.field_specification);
    if self.negative { ri.sign = !ri.sign; }
});

create_instruction!(StoreA, address: usize, field_specification: (usize, usize), (self, computer) {
    store_operation(&computer.ra, &mut computer.memory[self.address], self.field_specification);
});

create_instruction!(StoreX, address: usize, field_specification: (usize, usize), (self, computer) {
    store_operation(&computer.rx, &mut computer.memory[self.address], self.field_specification);
});

create_instruction!(StoreI, index: u8, address: usize, field_specification: (usize, usize), (self, computer) {
    let ri =  register_for_index(computer, self.index);
    let reg_clone = ri.clone();
    store_operation(
        &reg_clone, 
        &mut computer.memory[self.address], 
        self.field_specification
        );    
});

create_instruction!(StoreJ, address: usize, field_specification: (usize, usize), (self, computer) {
    store_operation(&computer.rj, &mut computer.memory[self.address], self.field_specification);
});

create_instruction!(StoreZ, address: usize, field_specification: (usize, usize), (self, computer) {
    let zero = Word::default();
    store_operation(&zero, &mut computer.memory[self.address], self.field_specification);
});

create_instruction!(Add, address: usize, field_specification: (usize, usize), (self, computer) {
    let (value, overflow) = add_words(&computer.ra, &computer.memory[self.address], self.field_specification);
    copy_word_fields(&value, &mut computer.ra, self.field_specification);
    computer.overflow_flag = overflow;
});

create_instruction!(Sub, address: usize, field_specification: (usize, usize), (self, computer) {
    let (value, overflow) = add_words(&computer.ra, &computer.memory[self.address].negate(), self.field_specification);
    copy_word_fields(&value, &mut computer.ra, self.field_specification);
    computer.overflow_flag = overflow;
});

create_instruction!(Mult, address: usize, field_specification: (usize, usize) , (self, computer) {
    let (lower_value, upper_value) = multiply_words(&computer.ra, &computer.memory[self.address].negate(), self.field_specification);
    copy_word_fields(&lower_value, &mut computer.rx, (0,5));
    copy_word_fields(&upper_value, &mut computer.ra, (0,5));
});

create_instruction!(Div, address: usize, field_specification: (usize, usize) , (self, computer) {
    let (dividend, remainder, overflow) = divide_words(&computer.ra, &computer.rx, &computer.memory[self.address].negate(), self.field_specification);
    copy_word_fields(&remainder, &mut computer.rx, (0,5));
    copy_word_fields(&dividend, &mut computer.ra, (0,5));
    computer.overflow_flag = overflow;
});

create_instruction!(EntA, value: usize, sign: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.sign = self.sign;
    copy_word_fields(&word, &mut computer.ra, (0, 5));
});