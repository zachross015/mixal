use crate::computer::{Computer, ComparisonFlag};
use crate::word::{Word};
use crate::instruction_functions::*;


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
    if self.negative { ra.positive = !ra.positive; }
});

create_instruction!(LoadX, address: usize, field_specification: (usize, usize), negative: bool, (self, computer) {
    let rx =  &mut computer.rx;
    let mem = &computer.memory[self.address];
    copy_word_fields(mem, rx, self.field_specification);
    if self.negative { rx.positive = !rx.positive; }
});

create_instruction!(LoadI, index: u8, address: usize, field_specification: (usize, usize), negative: bool, (self, computer) {
    let mem = &computer.memory[self.address].clone();
    let ri =  register_for_index(computer, self.index);
    copy_word_fields_i(mem, ri, self.field_specification);
    if self.negative { ri.positive = !ri.positive; }
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

create_instruction!(EntA, value: usize, entry_is_positive: bool, should_negate: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.positive = if self.should_negate { !self.entry_is_positive } else { self.entry_is_positive };
    copy_word_fields(&word, &mut computer.ra, (0, 5));
});

create_instruction!(EntX, value: usize, entry_is_positive: bool, should_negate: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.positive = if self.should_negate { !self.entry_is_positive } else { self.entry_is_positive };
    copy_word_fields(&word, &mut computer.rx, (0, 5));
});

create_instruction!(EntI, index: u8, value: usize, entry_is_positive: bool, should_negate: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.positive = if self.should_negate { !self.entry_is_positive } else { self.entry_is_positive };
    let mut ri =  register_for_index(computer, self.index);
    copy_word_fields_i(&word, &mut ri, (0,5));
});

create_instruction!(IncA, value: usize, entry_is_positive: bool, should_negate: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.positive = if self.should_negate { !self.entry_is_positive } else { self.entry_is_positive };
    let (value, overflow) = add_words(&computer.ra, &word, (0,5));
    copy_word_fields(&value, &mut computer.ra, (0, 5));
    computer.overflow_flag = overflow;
});

create_instruction!(IncX, value: usize, entry_is_positive: bool, should_negate: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.positive = if self.should_negate { !self.entry_is_positive } else { self.entry_is_positive };
    let (value, overflow) = add_words(&computer.rx, &word, (0,5));
    copy_word_fields(&value, &mut computer.rx, (0, 5));
    computer.overflow_flag = overflow;
});

create_instruction!(IncI, index: u8, value: usize, entry_is_positive: bool, should_negate: bool, (self, computer) {
    let mut word = Word::from_value(self.value as i64);
    word.positive = if self.should_negate { !self.entry_is_positive } else { self.entry_is_positive };
    let mut ri =  register_for_index(computer, self.index);
    let (value, overflow) = add_words(&ri, &word, (0,5));
    copy_word_fields(&value, &mut ri, (0, 5));
    computer.overflow_flag = overflow;
});

create_instruction!(CmpA, address: usize, field_specification: (usize, usize), (self, computer) {
    let result = compare_words(&computer.ra, &computer.memory[self.address], self.field_specification);
    computer.comparison_flag = result;
});

create_instruction!(CmpX, address: usize, field_specification: (usize, usize), (self, computer) {
    let result = compare_words(&computer.rx, &computer.memory[self.address], self.field_specification);
    computer.comparison_flag = result;
});

create_instruction!(CmpI, index: u8, address: usize, field_specification: (usize, usize), (self, computer) {
    let mem = computer.memory[self.address].clone();
    let ri =  register_for_index(computer, self.index);
    let result = compare_words(&ri, &mem, self.field_specification);
    computer.comparison_flag = result;
});

create_instruction!(Jmp, address: usize, save_address: bool, (self, computer) {
    if self.save_address {
        save_jump(computer);
    }
    computer.pc = self.address;
});

create_instruction!(JmpO, address: usize, should_negate: bool, (self, computer) {
    if computer.overflow_flag.clone() != self.should_negate {
        save_jump(computer);
        computer.pc = self.address;
    }
    computer.overflow_flag = false;
});

pub fn condition_match(op: u8, condition: ComparisonFlag) -> bool {
    match op {
        0 => condition == ComparisonFlag::less,
        1 => condition == ComparisonFlag::equal,
        2 => condition == ComparisonFlag::greater,
        3 => condition != ComparisonFlag::less,
        4 => condition != ComparisonFlag::equal,
        5=> condition != ComparisonFlag::greater,
        _ => false,
    }
}

create_instruction!(JmpC, address: usize, operation: u8, (self, computer) {
    let condition = condition_match(self.operation - 4, computer.comparison_flag);
    if condition {
        save_jump(computer);
        computer.pc = self.address;
    }
});

create_instruction!(JmpA, address: usize, operation: u8, (self, computer) {
    let zero = Word::default();
    let result = compare_words(&computer.ra, &zero, (0, 5));
    let condition = condition_match(self.operation, result);
    if condition {
        save_jump(computer);
        computer.pc = self.address;
    }
});

create_instruction!(JmpX, address: usize, operation: u8, (self, computer) {
    let zero = Word::default();
    let result = compare_words(&computer.rx, &zero, (0, 5));
    let condition = condition_match(self.operation, result);
    if condition {
        save_jump(computer);
        computer.pc = self.address;
    }
});

create_instruction!(JmpI, index: u8, address: usize, operation: u8, (self, computer) {
    let zero = Word::default();
    let ri =  register_for_index(computer, self.index);
    let result = compare_words(&ri, &zero, (0, 5));
    let condition = condition_match(self.operation, result);
    if condition {
        save_jump(computer);
        computer.pc = self.address;
    }
});

create_instruction!(SLA, amount: usize, cycle: bool, (self, computer) {
    let r = computer.ra.clone();
    computer.ra = single_word_left_shift(&r, self.amount, self.cycle);
});

create_instruction!(SRA, amount: usize, cycle: bool, (self, computer) {
    let r = computer.ra.clone();
    computer.ra = single_word_right_shift(&r, self.amount, self.cycle);
});

create_instruction!(SLAX, amount: usize, (self, computer) {
    let a = computer.ra.clone();
    let x = computer.rx.clone();
    let (ra, rx) = double_word_left_shift(&a, &x, self.amount);
    computer.ra = ra;
    computer.rx = rx;
});

create_instruction!(SRAX, amount: usize, (self, computer) {
    let a = computer.ra.clone();
    let x = computer.rx.clone();
    let (ra, rx) = double_word_right_shift(&a, &x, self.amount);
    computer.ra = ra;
    computer.rx = rx;
});
