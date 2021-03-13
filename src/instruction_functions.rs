use crate::word::Word;
use crate::computer::{Computer, ComparisonFlag};
use std::convert::TryInto;

/// Provides a useful macro for checking conditions involving adjusted field 
/// specifications. 
/// 
/// The `word_zero_condition` checks first if the value zero is included in 
/// the field specification. If it is, the positive of the receiving word is adjusted 
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
/// `positive` of the sending word to adjust the receiver if necessary. 
/// - `to_word`: The receiving word in the comparison. This is necessary 
/// so its `positive` value can be adjusted if zero is part of the adjusted field 
/// specification.
/// 
macro_rules! word_zero_condition {
    ($z:ident, $o:ident, $f:ident, $t:ident) => {
        if $z {
            $t.positive = $f.positive;
            if $o {
                return;
            }
        }
    }
}

/// Provides a useful conversion for the field specification of a MIX command. 
/// 
/// In the documentation for MIX, a word is laid out from left to right as the 
/// indices 0-6, with the 0-byte indicating the positive. In the program implementation,
/// we instead have the positive byte as just a boolean value, while the rest of the 
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
/// TODO: Document this
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
        word.positive = if sum >= 0 { true } else { false };
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

/// TODO: Document this
pub fn multiply_words(word1: &Word, word2: &Word, field_specification: (usize, usize)) -> (Word, Word) {
    let word1_value = word1.field_value((0,5)).clone();
    let word2_value = word2.field_value(field_specification);
    let mut word_lower = Word::default();
    let mut word_upper = Word::default();
    let mut product : i128 = (word1_value as i128) * (word2_value as i128);

    let (zero_included, only_zero, _) = adjusted_field_specification(field_specification);
    if only_zero {
        panic!("[Error multiply_words] Can't multiply two numbers solely by their positive. (Input given {:#?})", field_specification);
    }

    if zero_included {
        word_lower.positive = if product >= 0 { true } else { false };
        word_upper.positive = word_lower.positive;
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

/// TODO: Document this
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
        panic!("[Error divide_words] Can't divide two numbers solely by their positive. (Input given {:#?})", field_specification);
    }

    word_rem.positive = word1.positive;
    if zero_included {
        word_div.positive = word1.positive == word3.positive;
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

/// TODO: Document this
pub fn compare_words(word1: &Word, word2: &Word, field_specification: (usize, usize)) -> ComparisonFlag {
    let (zero_included, only_zero, (left, right)) = adjusted_field_specification(field_specification);

    if only_zero {
        return ComparisonFlag::equal;
    }

    if zero_included && word1.positive != word2.positive {
        if word1.positive == true {
            return ComparisonFlag::greater;
        } else {
            return ComparisonFlag::less;
        }
    }

    for i in left..=right {
        if word1.bytes[i] > word2.bytes[i] {
            return ComparisonFlag::greater;
        } else if word1.bytes[i] < word2.bytes[i] {
            return ComparisonFlag::less;
        }
    }

    ComparisonFlag::equal
}

// TODO: Document this <12-03-21, yourname> //
pub fn save_jump(computer: &mut Computer) {
    let old_address = Word::from_value((computer.pc.clone() + 1) as i64);    
    computer.rj = old_address;
}
/// Does a byte-wise left shift over a single word, performing the amount of shifts 
/// specified by `amount` and only retaining a cyclical shift if `cycle` is set to true.
///
/// ## Arguments
///
/// `word` - The reference to a `Word` that is being shifted to the left
/// `amount` - The `usize` indicating how many bytes the word should be shifted over
/// `cycle` - A `bool` value specifying whether or not shift should cycle bytes back to the
/// beginning that have gone before the beginning of the word boundary
pub fn single_word_left_shift(word: &Word, amount: usize, cycle: bool) -> Word {
    let mut r_copy = word.clone();
    for i in 0..5 {
        r_copy.bytes[i] = word.bytes[(amount + i) % 5];
    }
    if !cycle {
        for i in (5 - amount).max(0)..5 {
            r_copy.bytes[i] = 0;
        }
    }
    r_copy
}

/// Does a byte-wise right shift over a single word, performing the amount of shifts 
/// specified by `amount` and only retaining a cyclical shift if `cycle` is set to true.
///
/// ## Arguments
///
/// `word` - The reference to a `Word` that is being shifted to the right
/// `amount` - The `usize` indicating how many bytes the word should be shifted over
/// `cycle` - A `bool` value specifying whether or not shift should cycle bytes back to the
/// beginning that have gone past the end of the word boundary
pub fn single_word_right_shift(word: &Word, amount: usize, cycle: bool) -> Word {
    let mut r_copy = word.clone();
    for i in 0..5 {
        r_copy.bytes[(amount + i) % 5] = word.bytes[i];
    }
    if !cycle {
        for i in 0..amount.min(5) {
            r_copy.bytes[i] = 0;
        }
    }
    r_copy
}

/// Does a byte-wise left shift over two words, performing the amount of shifts specified by
/// `amount`. This acts on the two words by seeing each of their individual bytes as being
/// consecutive, and performing a left shift as if this was a 10 byte word.  The sign of each word
/// is not altered and any bytes to the left of the farthest shifting byte will be set to 0.
///
/// ## Arguments
///
/// `word1` - The reference to the upper `Word` that is being shifted to the left
/// `word2` - The reference to the lower `Word` that is being shifted to the left
/// `amount` - The `usize` indicating how many bytes the word should be shifted over
pub fn double_word_left_shift(word1: &Word, word2: &Word, amount: usize) -> (Word, Word) {
    let mut w1_copy = word1.clone();
    let mut w2_copy = word2.clone();
    let mut vals = [0; 10];

    // Set up `vals` to contain the 10 bytes of consecutive data
    for i in 0..5 {
        vals[i] = w1_copy.bytes[i];
    }
    for i in 5..10 {
        vals[i] = w2_copy.bytes[i - 5];
    }

    // Set up `vals_shifted` to contain the modulus shifting of each byte
    let mut vals_shifted = [0; 10];
    if amount < 10 {
        for i in 0..(10 - amount) {
            vals_shifted[i] = vals[(amount + i) % 10];
        }
    }

    // Set any excess bytes to 0
    for i in (10 - amount).max(0)..10 {
        vals_shifted[i] = 0;
    }

    // Slice the modulus shifted values into the byte arrays for each word, and set 
    // each word to the corresponding value
    w1_copy.bytes = vals_shifted[0..5].try_into().expect("Tried slice with incorrect length.");
    w2_copy.bytes = vals_shifted[5..10].try_into().expect("Tried slice with incorrect length.");
    (w1_copy, w2_copy)
}

/// Does a byte-wise right shift over two words, performing the amount of shifts specified by
/// `amount`. This acts on the two words by seeing each of their individual bytes as being
/// consecutive, and performing a right shift as if this was a 10 byte word.  The sign of each word
/// is not altered and any bytes to the right of the farthest shifting byte will be set to 0.
///
/// ## Arguments
///
/// `word1` - The reference to the upper `Word` that is being shifted to the right
/// `word2` - The reference to the lower `Word` that is being shifted to the right
/// `amount` - The `usize` indicating how many bytes the word should be shifted over
pub fn double_word_right_shift(word1: &Word, word2: &Word, amount: usize) -> (Word, Word) {
    let mut w1_copy = word1.clone();
    let mut w2_copy = word2.clone();
    let mut vals = [0; 10];

    // Set up `vals` to contain the 10 bytes of consecutive data
    for i in 0..5 {
        vals[i] = w1_copy.bytes[i];
    }
    for i in 5..10 {
        vals[i] = w2_copy.bytes[i - 5];
    }

    // Set up `vals_shifted` to contain the modulus shifting of each byte
    let mut vals_shifted = [0; 10];
    if amount < 10 {
        for i in 0..(10 - amount) {
            vals_shifted[(amount + i) % 10] = vals[i];
        }
    }

    // Set any excess bytes to 0
    for i in 0..amount.min(10) {
        vals_shifted[i] = 0;
    }

    // Slice the modulus shifted values into the byte arrays for each word, and set 
    // each word to the corresponding value
    w1_copy.bytes = vals_shifted[0..5].try_into().expect("Tried slice with incorrect length.");
    w2_copy.bytes = vals_shifted[5..10].try_into().expect("Tried slice with incorrect length.");
    (w1_copy, w2_copy)
}
