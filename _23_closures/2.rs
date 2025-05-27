// Lecture: Closure introduction

fn main() {
    let multipler = 5;

    let multiply_by = |value: i32| {
        return value * multipler
        // and since value implement the copy trait
        // we're have a full copy of 5 into our clousure scope
    };

    println!("{}", multiply_by(2));

    let product = |a: i32, b:i32| -> i32 {
        println!("Calculating product for you");
        return a * b + multipler
    };

    println!("{}", product(3, 2));
}
