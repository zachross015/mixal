
mod word;
mod computer;
mod instruction;

#[cfg(test)]
mod tests;

use crate::word::{Word};
use crate::computer::Computer;

fn main() {

    let w1 = Word::new(false, [1,2,3,4,5]);
    let w2 = Word::new(true, [0,0,0,9,1]);
    let w1val = w1.field_value((0,3));
    let w2val = w2.field_value((0,3));
    println!("{} + {} = {}", w1val, w2val, w1val + w2val);
    println!("Hello world!");
}

