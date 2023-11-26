// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = 0..100;
    //Or let a = ["Hello"; 100];

    // 1..2;   // std::ops::Range
    // 3..;    // std::ops::RangeFrom
    // ..4;    // std::ops::RangeTo
    // ..;     // std::ops::RangeFull
    // 5..=6;  // std::ops::RangeInclusive
    // ..=7;   // std::ops::RangeToInclusive

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
