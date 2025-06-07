// Lecture: Closure introductio

fn main() {
    // this is a closure that returns a i32, we can omit the curly braces
    // and also, we can explicity annotate its return types like this:
    // let multipler = || 5 as u8;
    let multipler = || 5;

    // and since value implement the copy trait
    // we're have a full copy of 5 into our clousure scope
    // and the value argument is inferred by rust. With functions
    // it will inferred as unit but in closures it infers base
    // on the body of it.
    let multiply_by = |value| { value * multipler() };
    // also, rust will infer the parameter type by first time invocation.
    // println!("{}", multiply_by(2 as u8));
    println!("{}", multiply_by(2)); // i32
    // after first invocation, its param type will persist

    // example with multiple arguments
    // let product = |a: i32, b:i32| -> i32 {
    //     println!("Calculating product for you");
    //     return a * b + multipler
    // };

    // println!("{}", product(3, 2));
}

// functions and its typifications are more strict because it's meant to be
// used for any other section code rather closures is just for a one direct purpose 
