// Multiple generics
fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

fn main() {
    make_tuple("David", 33);
}