// The turbofish operator
// It used for explicit type declaration
// in previous lecture type was infered by the compiler
fn identity<T>(value: T) -> T {
    value
}

#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity::<f32>(5.99));
    println!("{}", identity::<&str>("5.99"));
    println!("{}", identity::<&str>("hello"));
    println!("{}", identity::<String>(String::from("Example")));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich{}));
}