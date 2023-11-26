// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // Use the reference of a, and construct a slice with indexes 1-3 non-inclusive of the final element
    // let nice_slice = &a[1..4];

    // Or construct a a new array by accesing the values of a directly
    let nice_slice = [a[1], a[2], a[3]];

    assert_eq!([2, 3, 4], nice_slice)
}
