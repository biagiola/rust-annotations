fn main() {
    // three of them have its own reference in the stack
    let coffe: String = String::from("Mocha");
    let a: &String = &coffe;
    let b: &String = a;

    println!("{:p} {}", &coffe, coffe);
    println!("{:p} {}", &a, a);
    println!("{:p} {}", &b, b);
}