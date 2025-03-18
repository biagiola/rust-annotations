fn main() {
    let mut values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let regular_reference: &[i32; 6] = &values;
    let slice_of_three: &[i32] = &values[..3];

    values[0] = 99; // cannot modify while an immutable borrow exists
}
