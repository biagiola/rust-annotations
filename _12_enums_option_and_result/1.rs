// The Option enum models a sceneario
// where a type could be a valid value
// or nothing at all.
 
// possible values
// Option::Some(T) represents a present value.
// Option::None represents an absent value.

fn main() {
    let a: Option<i8> = Option::Some(32);
    let b: Option<&str> = Option::Some("Hello");
    let c: Option<bool> = Option::Some(true);

    // both are the same
    let a: Option<i8> = Option::Some(5);
    let a: Option<i8> = Option::<i8>::Some(5); // it seams redundant.. ?

    // non version
    let d: Option<&str> = Option::None;
}
