fn main() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 43];

    let my_slice: &[i32] = &values[..4];
    println!("{my_slice:?}"); // 4, 8, 15, 16

    let my_slice: &[i32] = &values[2..4];
    println!("{my_slice:?}"); // 15, 16

    let my_slice: &[i32] = &values[2..];
    println!("{my_slice:?}"); // 15, 16, 23, 43 

    let my_slice: &[i32] = &values[..];
    println!("{my_slice:?}"); // 4, 8, 15, 16, 23, 43

    let my_slice: &[i32] = &values;
    println!("{my_slice:?}"); // 4, 8, 15, 16, 23, 43
}