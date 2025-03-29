fn identity<T>(value: T) -> T {
    value
}

#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(5.99));
    println!("{}", identity("5.99"));
    println!("{}", identity("hello"));
    println!("{}", identity(String::from("Example")));
    println!("{:?}", identity(DeliSandwich {}));
}

// for the compiler, we have multiple identity fn's for each type like previous lecture,
// and generics doesn't exists under the hood.
// This term is called monomorphization