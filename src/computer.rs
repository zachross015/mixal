use std::fmt;
use crate::word::{Word};
use crate::instruction::*;

macro_rules! boxed {
    ($name:ident) => {
        Box::new($name::new())
    };
    ($name:ident, $($item:ident),*) => {
        Box::new($name::new($($item),*))
    };
    ($name:ident, $expr:expr, $($item:ident),*) => {
        Box::new($name::new($expr, $($item),*))
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ComparisonFlag {
    less,
    equal,
    greater
}

#[derive(Copy, Clone)]
pub struct Computer {
    pub ra: Word,
    pub rx: Word, 
    pub ri1: Word, 
    pub ri2: Word,
    pub ri3: Word,
    pub ri4: Word,
    pub ri5: Word,
    pub ri6: Word,
    pub rj: Word,
    pub overflow_flag: bool,
    pub comparison_flag: ComparisonFlag,
    pub memory: [Word; 4000],
    pub peripherals: [u8; 20],
    pub pc: usize,
}

impl Computer {

    pub fn new(mem: [Word; 4000], start: usize) -> Computer {
        Computer {
            ra: Word::default(),
            rx: Word::default(), 
            ri1: Word::default(), 
            ri2: Word::default(),
            ri3: Word::default(),
            ri4: Word::default(),
            ri5: Word::default(),
            ri6: Word::default(),
            rj: Word::default(),
            overflow_flag: false,
            comparison_flag: ComparisonFlag::equal,
            memory: mem,
            peripherals: [0; 20],
            pc: start,
        }
    }

    pub fn default() -> Computer {
        Computer::new([Word::default(); 4000], 0)
    }

    fn fetch(self) -> Word {
        self.memory[self.pc]
    }

    fn decode_index(&mut self, index: &u8) -> usize {
        if *index == 0 {
            return 0;
        }
        let ri = register_for_index(self, *index);
        ri.field_value((3, 4)) as usize
    }

    fn decode_field(&self, field: &u8) -> (usize, usize) {
        let right = field % 8;
        let left = field / 8;
        (left as usize, right as usize)
    }

    fn decode(&mut self, instruction: &Word) ->  Box<dyn Instruction> {
        let (address, index, field, opcode) = (instruction.address(), instruction.index(), instruction.field(), instruction.opcode());

        // Handle the index register
        let offset_address = address + self.decode_index(&index);
        let field_specification = self.decode_field(&field);
        let positive = instruction.positive;
        let field = instruction.field();


        let inst : Box<dyn Instruction> = match opcode {
            1 => boxed!(Add, offset_address, field_specification),
            2 => boxed!(Sub, offset_address, field_specification),
            3 => boxed!(Mult, offset_address, field_specification),
            4 => boxed!(Div, offset_address, field_specification),
            5 => match field {
                2 => boxed!(Halt),
                _ => boxed!(NoOperation)
            }
            8 => boxed!(LoadA, offset_address, field_specification, false),
            9 | 10 | 11 | 12 | 13 | 14 => boxed!(LoadI, opcode - 8, offset_address, field_specification, false),
            15 => boxed!(LoadX, offset_address, field_specification, false),
            16 => boxed!(LoadA, offset_address, field_specification, true),
            17 | 18 | 19 | 20 | 21 | 22 => boxed!(LoadI, opcode - 16, offset_address, field_specification, true),
            23 =>boxed!(LoadX, offset_address, field_specification, true),
            24 => boxed!(StoreA, offset_address, field_specification),
            25 | 26 | 27 | 28 | 29 | 30 => boxed!(StoreI, opcode - 24, offset_address, field_specification),
            31 => boxed!(StoreX, offset_address, field_specification),
            32 => boxed!(StoreJ, offset_address, field_specification),
            33 => boxed!(StoreZ, offset_address, field_specification),
            39 => match field {
                0 => boxed!(Jmp, address, true),
                1 => boxed!(Jmp, address, false),
                2 => boxed!(JmpO, address, false),
                3 => boxed!(JmpO, address, true),
                4 | 5 | 6 | 7 | 8 | 9 => boxed!(JmpC, address, field),
                _ => boxed!(NoOperation),
            },
            48 => match field {
                0 => boxed!(IncA, offset_address, positive, false),
                1 => boxed!(IncA, offset_address, positive, true),
                2 => boxed!(EntA, offset_address, positive, false),
                3 => boxed!(EntA, offset_address, positive, true),
                _ => boxed!(NoOperation),
            },
            49 | 50 | 51 | 52 | 53 | 54 => match field {
                0 => boxed!(IncI, opcode - 48, offset_address, positive, false),
                1 => boxed!(IncI, opcode - 48, offset_address, positive, true),
                2 => boxed!(EntI, opcode - 48, offset_address, positive, false),
                3 => boxed!(EntI, opcode - 48, offset_address, positive, true),
                _ => boxed!(NoOperation),
            },
            55 => match field {
                0 => boxed!(IncX, offset_address, positive, false),
                1 => boxed!(IncX, offset_address, positive, true),
                2 => boxed!(EntX, offset_address, positive, false),
                3 => boxed!(EntX, offset_address, positive, true),
                _ => boxed!(NoOperation),
            },
            56 => boxed!(CmpA, offset_address, field_specification),
            57 | 58 | 59 | 60 | 61 | 62 => boxed!(CmpI, opcode - 56, offset_address, field_specification),
            63 => boxed!(CmpX, offset_address, field_specification),
            _ => boxed!(NoOperation),
        };

        inst
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            let decoded_instruction = self.decode(&instruction);
            decoded_instruction.execute_on(self);
            if self.pc == 4000 { break }
            self.pc = self.pc + 1;
        }
    }

}
