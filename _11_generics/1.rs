// A generci is a type argument. It's like a placeholder for a future type.
// A generic is like a parameter, but for a type like i32, String or bool

fn identity_i32(value: i32) -> i32 {
    value
}

fn identity_bool(value: bool) -> bool {
    value
}
// both fn's are almost identical but differs in its return type value
// with generic we can implement a generic way to handle this scenario.
fn identity<T>(value: T) -> T {
    value
}

fn main() {}