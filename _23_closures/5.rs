// Lecture: FnMut traits closures

fn main() {
    // we're show heap data example, the same apply for stack variables
    let mut numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);

    // We cannot make a print of 'number' because we can only have one
    // reference at the same time when we're dealing with mutable refs
    // println!("{:?}", numbers); // now allowed

    add_number();
    add_number();

    // here, now is free to use numbers. In this case, use means reads
    // but for the compiler could be also modify its value, and we don't
    // want to encounter any collisions, that's why mutable refs can only
    // have one reference at the time. 
    println!("{:?}", numbers);
}