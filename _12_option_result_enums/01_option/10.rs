// The Option enum models a scenario where a type could be a valid value or nothing at all

// possible values
// Option::None represents an absent value
// Option::Some represents a present value

fn main() {
    let a: Option<i8> = Option::Some(32);
    let b: Option<&str> = Option::Some("hello");
    let c: Option<bool> = Option::Some(true);
    
    // both are the same
    let a: Option<i8> = Option::Some(5);
    let a: Option<i8> = Option::<i8>::Some(5);
    
    // the non versions
    let d: Option<&str> = Option::None;
}