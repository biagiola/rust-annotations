fn main() {
    let fifty_numbers = 1..=50;

    // for number in fifty_numbers.take(15) {
    //     println!("{number}/");
    // }

    // for number in fifty_numbers.rev().take(15) {
    //     print!("{number}/");
    // }

    // for number in fifty_numbers.skip(5).take(15) {
    //     print!("{number}/");
    // }

    // for number in fifty_numbers.take(15).skip(5) {
    //     print!("{number}/");
    // }

    for number in fifty_numbers.take(15).skip(5).step_by(2) {
        print!("{number}/");
    }

    // all the methods example move ownership
    // and there is not iter() method on a range.
    // to conserve ownership we need to use the clone method
    for number in fifty_numbers.clone().take(15).skip(5).step_by(2) {
        print!("{number}/");
    }
}