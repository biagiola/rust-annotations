fn main() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    // this is just a reference, not a slice technically
    let regular_reference: &[i32; 6] = &values;

    // this is a slice and have more versatility coz
    // the non length specification
    let slice_of_three: &[i32] = &values[..3];
    println!("{:?}", slice_of_three);
}
