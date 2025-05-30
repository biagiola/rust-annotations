fn main() {
    let mut numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);

    // println!("{:?}", numbers); // now allowed, just one mutable ref at the same time
    add_number();
    add_number();
    println!("{:?}", numbers);
}