use crate::word::{Word};
use crate::computer::*;
use crate::instruction::*;
use crate::instruction_functions::*;
use rand::Rng;

const ADDRESS: usize = 2000;

fn sample_mem() -> Word { Word::new(false, [1,2,3,4,5]) }
fn sample_reg() -> Word { Word::new(true, [0,0,0,9,1]) }

#[test] 
fn copy_word_fields_full() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let mut word2 = Word::default();
    let left = 0;
    let right = 5;
    copy_word_fields(&word1, &mut word2, (left, right));
    println!("[{}] [{}]", word1, word2);
    assert_eq!(word1, word2);
}

#[test]
fn copy_word_fields_partial() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let mut word2 = Word::default();
    let left = 0;
    let right = 2;
    copy_word_fields(&word1, &mut word2, (left, right));

    assert_eq!(word1.positive, word2.positive);
    for i in 0..=1 {
        assert_eq!(word1.bytes[i], word2.bytes[i]);
    }
    for i in 2..=4 {
        assert_ne!(word1.bytes[i], word2.bytes[i]);
    }
}

#[test]
fn copy_word_fields_single() {
    let word1 = Word::new(false, [1,1,1,1,1]);
    let mut word2 = Word::default();
    let left = 2;
    let right = 2;
    copy_word_fields(&word1, &mut word2, (left, right));

    assert_ne!(word1.positive, word2.positive);
    for i in 0..=0 {
        assert_ne!(word1.bytes[i], word2.bytes[i]);
    }
    assert_eq!(word1.bytes[1], word2.bytes[1]);
    for i in 2..=4 {
        assert_ne!(word1.bytes[i], word2.bytes[i]);
    }
}

fn excluse_bits_equivalent_full(word1: &Word, word2: &Word, (begin, end): (usize, usize)) {
    if begin == 0 {
        assert_eq!(word1.positive, word2.positive);
    } else {
        assert_ne!(word1.positive, word2.positive);
    }
    for i in 0..=4 {
        if (begin..=end).contains(&(i + 1)) {
            assert_eq!(word1.bytes[i], word2.bytes[i]);
        } else {
            assert_ne!(word1.bytes[i], word2.bytes[i]);
        }
    }
}


fn excluse_bits_equivalent_index(word1: &Word, word2: &Word, (begin, end): (usize, usize)) {
    let (l, r) = (begin.max(1) - 1, end.max(1) - 1);
    if begin == 0 {
        assert_eq!(word1.positive, word2.positive);
    } else {
        assert_ne!(word1.positive, word2.positive);
    }

    for i in 0..=2 {
        assert_ne!(word1.bytes[i], word2.bytes[i]);
    }

    if (l..=r).contains(&3) {
        assert_eq!(word1.bytes[3], word2.bytes[3]);
    } else {
        assert_ne!(word1.bytes[3], word2.bytes[3]);
    }

    if (l..=r).contains(&4) {
        assert_eq!(word1.bytes[4], word2.bytes[4]);
    } else {
        assert_ne!(word1.bytes[4], word2.bytes[4]);
    }
    
}

fn rand_fill_range(word: &mut Word, begin: usize, end: usize) {
    let mut gen = rand::thread_rng();
    for i in begin..=end {
        word.bytes[i] = gen.gen_range(1,100);
    }
}

fn test_load_a_with_range(begin: usize, end: usize) {
    
    let computer = &mut Computer::default();
    let load = LoadA {
        address: ADDRESS,
        field_specification: (begin, end),
        negative: false
    };
    rand_fill_range(&mut computer.memory[ADDRESS], 0, 4);
    computer.memory[ADDRESS].positive = false;
    load.execute_on(computer);
    println!("[{}] [{}]", computer.memory[ADDRESS], computer.ra);
    excluse_bits_equivalent_full(&computer.ra, &computer.memory[ADDRESS], load.field_specification);
}

fn test_load_i_with_range(begin: usize, end: usize) {
    let load = LoadI {
        index: 1,
        address: ADDRESS,
        field_specification: (begin, end),
        negative: false,
    };
    let computer = &mut Computer::default();
    rand_fill_range(&mut computer.memory[ADDRESS], 0, 4);
    computer.memory[ADDRESS].positive = false;
    load.execute_on(computer);
    println!("[{}] [{}]", computer.memory[ADDRESS], computer.ri1);
    excluse_bits_equivalent_index(&computer.ri1, &computer.memory[ADDRESS], load.field_specification);
}

#[test]
fn load_a_full() {
    test_load_a_with_range(0, 5);
}

#[test]
fn load_a_1_4() {
    test_load_a_with_range(1, 5);
}

#[test]
fn load_a_3_5() {
    test_load_a_with_range(3, 5);
}

#[test]
fn load_a_0_3() {
    test_load_a_with_range(0, 3);
}

#[test]
fn load_a_2_2() {
    test_load_a_with_range(2, 2);
}

#[test]
fn load_i_full() {
    test_load_i_with_range(0, 5);
}

#[test]
fn load_i_3_5() {
    test_load_i_with_range(3, 5);
}

#[test]
fn load_i_0_3() {
    test_load_i_with_range(0, 3);
}

#[test]
#[should_panic]
fn load_i_1_3() {
    test_load_i_with_range(1, 3);
}



fn make_store_a_with_range(begin: usize, end: usize) -> StoreA {
    StoreA::new(ADDRESS, (begin, end))
}

fn store_a_test_setup(begin: usize, end: usize) -> Computer { 
    let mut computer = Computer::default();
    computer.ra = sample_reg();
    computer.memory[ADDRESS] =  sample_mem();
    let store = make_store_a_with_range(begin, end);
    store.execute_on(&mut computer);
    computer
}

#[test]
fn store_a_full() {
    let computer = store_a_test_setup(0, 5);
    let should_be = Word::new(true, [0,0,0,9,1]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_a_1_5() {
    let computer = store_a_test_setup(1, 5);
    let should_be = Word::new(false, [0,0,0,9,1]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_a_5_5() {
    let computer = store_a_test_setup(5, 5);
    let should_be = Word::new(false, [1,2,3,4,1]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_a_2_2() {
    let computer = store_a_test_setup(2, 2);
    let should_be = Word::new(false, [1,1,3,4,5]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_a_2_3() {
    let computer = store_a_test_setup(2, 3);
    let should_be = Word::new(false, [1,9,1,4,5]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_a_0_1() {
    let computer = store_a_test_setup(0, 1);
    let should_be = Word::new(true, [1,2,3,4,5]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

fn make_store_i_with_range(begin: usize, end: usize) -> StoreI {
    StoreI::new(1, ADDRESS, (begin, end))
}

fn store_i_test_setup(begin: usize, end: usize) -> Computer { 
    let mut computer = Computer::default();
    computer.ri1 = sample_reg();
    computer.memory[ADDRESS] =  sample_mem();
    let store = make_store_i_with_range(begin, end);
    store.execute_on(&mut computer);
    computer
}

#[test]
fn store_i_full() {
    let computer = store_i_test_setup(0, 5);
    let should_be = Word::new(true, [0,0,0,9,1]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_i_1_5() {
    let computer = store_i_test_setup(1, 5);
    let should_be = Word::new(false, [0,0,0,9,1]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_i_5_5() {
    let computer = store_i_test_setup(5, 5);
    let should_be = Word::new(false, [1,2,3,4,1]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_i_2_2() {
    let computer = store_i_test_setup(2, 2);
    let should_be = Word::new(false, [1,1,3,4,5]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_i_2_3() {
    let computer = store_i_test_setup(2, 3);
    let should_be = Word::new(false, [1,9,1,4,5]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

#[test]
fn store_i_0_1() {
    let computer = store_i_test_setup(0, 1);
    let should_be = Word::new(true, [1,2,3,4,5]);
    println!("{} {}", computer.memory[ADDRESS], should_be);
    assert_eq!(computer.memory[ADDRESS], should_be);
}

fn make_add_with_range(begin: usize, end: usize) -> Add {
    Add::new(ADDRESS, (begin, end))
}
fn add_test_setup(begin: usize, end: usize) -> Computer { 
    let mut computer = Computer::default();
    computer.ra = sample_reg();
    computer.memory[ADDRESS] =  sample_mem();
    let store = make_add_with_range(begin, end);
    store.execute_on(&mut computer);
    computer
}

#[test]
fn add_full() {
    let computer = add_test_setup(0, 5);
    let should_be = Word::new(false, [1,2,2,251,4]);
    println!("{} {} {} {}", 
        computer.ra, 
        should_be, 
        computer.ra.field_value((0,5)), 
        should_be.field_value((0,5))
    );
    assert_eq!(computer.ra, should_be);
}

#[test]
fn add_1_5() {
    let computer = add_test_setup(1, 5);
    let should_be = Word::new(true, [1,2,3,13,6]);
    println!("{} {}", computer.ra, should_be);
    assert_eq!(computer.ra, should_be);
}

#[test]
fn add_0_3() {
    let computer = add_test_setup(0, 3);
    let should_be = Word::new(false, [1,2,3,9,1]);
    println!("{} {}", computer.ra, should_be);
    assert_eq!(computer.ra, should_be);
}

#[test]
fn add_4_5() {
    let computer = add_test_setup(4, 5);
    let should_be = Word::new(true, [0,0,0,13,6]);
    println!("{} {}", computer.ra, should_be);
    assert_eq!(computer.ra, should_be);
}

#[test]
#[should_panic]
fn add_0_0() {
    let computer = add_test_setup(0, 0);
    let should_be = Word::new(true, [0,0,0,13,6]);
    println!("{} {}", computer.ra, should_be);
    assert_eq!(computer.ra, should_be);
}


#[test]
fn mult_full() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let word2 = Word::new(true, [1,1,1,1,1]);
    let output = multiply_words(&word1, &word2, (0, 5));
    let should_be = (Word::new(true, [0,1,2,3,4]), Word::new(true, [5,4,3,2,1]));
    println!("{:#?} {:#?}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn mult_neg() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let word2 = Word::new(false, [1,1,1,1,1]);
    let output = multiply_words(&word1, &word2, (0, 5));
    let should_be = (Word::new(false, [0,1,2,3,4]), Word::new(false, [5,4,3,2,1]));
    println!("{:#?} {:#?}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn mult_2_2() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let word2 = Word::new(true, [1,2,1,1,1]);
    let output = multiply_words(&word1, &word2, (2, 2));
    let should_be = (Word::new(true, [0,0,0,0,0]), Word::new(true, [2,2,2,2,2]));
    println!("{:#?} {:#?}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn div_full() {
    let word_a = Word::new(true, [0,0,0,0,0]);
    let word_x = Word::new(false, [0,0,0,0,17]);
    let word_div = Word::new(true, [0,0,0,0,3]);
    let output = divide_words(&word_a, &word_x, &word_div, (0,5));
    let should_be = (Word::new(true, [0,0,0,0,5]), Word::new(true, [0,0,0,0,2]), false);
    println!("{:#?} {:#?}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn enta_1000() {
    let mut computer = Computer::default();
    let instruction = EntA::new(1000, true, false);
    instruction.execute_on(&mut computer);
    println!("{:#?} {:#?}", Word::new(true, [0,0,0,3,232]), computer.ra);
    assert_eq!(Word::new(true, [0,0,0,3,232]), computer.ra);
}


#[test]
fn enta_neg_0() {
    let mut computer = Computer::default();
    let instruction = EntA::new(0, false, false);
    instruction.execute_on(&mut computer);
    println!("{:#?} {:#?}", Word::new(false, [0,0,0,0,0]), computer.ra);
    assert_eq!(Word::new(false, [0,0,0,0,0]), computer.ra);
}

#[test]
fn inc_1_1() {
    let mut computer = Computer::default();
    let instruction = IncI::new(1, 1, true, false);
    instruction.execute_on(&mut computer);
    println!("{:#?} {:#?}", Word::new(true, [0,0,0,0,1]), computer.ri1);
    assert_eq!(Word::new(true, [0,0,0,0,1]), computer.ri1);
}

#[test]
fn dec_1_1() {
    let mut computer = Computer::default();
    let instruction = IncI::new(1, 1, true, true);
    instruction.execute_on(&mut computer);
    println!("{:#?} {:#?}", Word::new(false, [0,0,0,0,1]), computer.ri1);
    assert_eq!(Word::new(false, [0,0,0,0,1]), computer.ri1);
}

#[test]
fn cmpa_0_0() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let word2 = Word::new(false, [1,1,1,1,1]);
    let output = compare_words(&word1, &word2, (0, 0));
    let should_be = ComparisonFlag::equal;
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn cmpa_full() {
    let word1 = Word::new(true, [1,1,1,1,1]);
    let word2 = Word::new(false, [1,1,1,1,1]);
    let output = compare_words(&word1, &word2, (0, 5));
    let should_be = ComparisonFlag::greater;
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn cmpa_full_less() {
    let word2 = Word::new(true, [1,1,1,1,1]);
    let word1 = Word::new(false, [1,1,1,1,1]);
    let output = compare_words(&word1, &word2, (0, 5));
    let should_be = ComparisonFlag::less;
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn cmpa_1_5() {
    let word1 = Word::new(false, [1,1,1,1,1]);
    let word2 = Word::new(true, [1,1,1,1,1]);
    let output = compare_words(&word1, &word2, (1, 5));
    let should_be = ComparisonFlag::equal;
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}


#[test]
fn single_word_left_shift_1() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_left_shift(&word1, 1, false);
    let should_be = Word::new(false, [2,3,4,5,0]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_left_shift_3() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_left_shift(&word1, 3, false);
    let should_be = Word::new(false, [4,5,0,0,0]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_left_shift_5() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let output = single_word_left_shift(&word1, 5, false);
    let should_be = Word::default();
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_left_shift_cycle_1() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_left_shift(&word1, 1, true);
    let should_be = Word::new(false, [2,3,4,5,1]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_left_shift_cycle_3() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_left_shift(&word1, 3, true);
    let should_be = Word::new(false, [4,5,1,2,3]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_left_shift_cycle_5() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let output = single_word_left_shift(&word1, 5, true);
    let should_be = word1.clone();
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_right_shift_1() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_right_shift(&word1, 1, false);
    let should_be = Word::new(false, [0,1,2,3,4]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_right_shift_3() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_right_shift(&word1, 3, false);
    let should_be = Word::new(false, [0,0,0,1,2]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_right_shift_5() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let output = single_word_right_shift(&word1, 5, false);
    let should_be = Word::default();
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_right_shift_cycle_1() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_right_shift(&word1, 1, true);
    let should_be = Word::new(false, [5,1,2,3,4]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_right_shift_cycle_3() {
    let word1 = Word::new(false, [1,2,3,4,5]);
    let output = single_word_right_shift(&word1, 3, true);
    let should_be = Word::new(false, [3,4,5,1,2]);
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn single_word_right_shift_cycle_5() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let output = single_word_right_shift(&word1, 5, true);
    let should_be = word1.clone();
    println!("{:} {:}", output, should_be);
    assert_eq!(output, should_be);
}

#[test]
fn save_jump_test() {
    let mut computer = Computer::default();
    computer.pc = 20;
    save_jump(&mut computer);
    assert_eq!(Word::new(true, [0,0,0,0,21]), computer.rj);
}

#[test]
fn double_word_left_shift_1() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let word2 = Word::new(true, [6,7,8,9,0]);
    let (output1, output2) = double_word_left_shift(&word1, &word2, 1);
    let should_be1 = Word::new(true, [2,3,4,5,6]);
    let should_be2 = Word::new(true, [7,8,9,0,0]);
    println!("{:} {:}", output1, should_be1);
    println!("{:} {:}", output2, should_be2);
    assert_eq!(output1, should_be1);
    assert_eq!(output2, should_be2);
}

#[test]
fn double_word_left_shift_3() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let word2 = Word::new(true, [6,7,8,9,0]);
    let (output1, output2) = double_word_left_shift(&word1, &word2, 3);
    let should_be1 = Word::new(true, [4,5,6,7,8]);
    let should_be2 = Word::new(true, [9,0,0,0,0]);
    println!("{:} {:}", output1, should_be1);
    println!("{:} {:}", output2, should_be2);
    assert_eq!(output1, should_be1);
    assert_eq!(output2, should_be2);
}

#[test]
fn double_word_left_shift_5() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let word2 = Word::new(true, [6,7,8,9,0]);
    let (output1, output2) = double_word_left_shift(&word1, &word2, 5);
    let should_be1 = Word::new(true, [6,7,8,9,0]);
    let should_be2 = Word::new(true, [0,0,0,0,0]);
    println!("{:} {:}", output1, should_be1);
    println!("{:} {:}", output2, should_be2);
    assert_eq!(output1, should_be1);
    assert_eq!(output2, should_be2);
}

#[test]
fn double_word_right_shift_1() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let word2 = Word::new(true, [6,7,8,9,0]);
    let (output1, output2) = double_word_right_shift(&word1, &word2, 1);
    let should_be1 = Word::new(true, [0,1,2,3,4]);
    let should_be2 = Word::new(true, [5,6,7,8,9]);
    println!("{:} {:}", output1, should_be1);
    println!("{:} {:}", output2, should_be2);
    assert_eq!(output1, should_be1);
    assert_eq!(output2, should_be2);
}

#[test]
fn double_word_right_shift_3() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let word2 = Word::new(true, [6,7,8,9,0]);
    let (output1, output2) = double_word_right_shift(&word1, &word2, 3);
    let should_be1 = Word::new(true, [0,0,0,1,2]);
    let should_be2 = Word::new(true, [3,4,5,6,7]);
    println!("{:} {:}", output1, should_be1);
    println!("{:} {:}", output2, should_be2);
    assert_eq!(output1, should_be1);
    assert_eq!(output2, should_be2);
}

#[test]
fn double_word_right_shift_5() {
    let word1 = Word::new(true, [1,2,3,4,5]);
    let word2 = Word::new(true, [6,7,8,9,0]);
    let (output1, output2) = double_word_right_shift(&word1, &word2, 5);
    let should_be1 = Word::new(true, [0,0,0,0,0]);
    let should_be2 = Word::new(true, [1,2,3,4,5]);
    println!("{:} {:}", output1, should_be1);
    println!("{:} {:}", output2, should_be2);
    assert_eq!(output1, should_be1);
    assert_eq!(output2, should_be2);
}
