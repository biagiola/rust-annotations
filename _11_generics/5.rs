// Multiple generics
fn make_tuple<T>(first: T, second: i32) -> (T, i32) { // just one of them has generic type
    (first, second)
}

fn make_tuple_2<T>(first: T, second: T) -> (T, T) { // both share same type
    (first, second)
}

fn make_tuple_3<T, U>(first: T, second: U) -> (T, U) { // both has its own type
    (first, second)
}

fn main() {
    make_tuple("David", 33);
    make_tuple_2("David", "Biagiola");
    make_tuple_3("David", 33);
    make_tuple_3("David", "B"); // no need to differ necessarily
}
