// The 'drop' function takes ownership of any value and calls its Drop trait
// implementation to clean up resources (like heap memory, file handles, etc.).
// You can call 'drop' on both heap and stack allocated variables.
// For stack variables like integers, drop just takes ownership and the value
// goes out of scope immediately (no special cleanup needed).
fn main() {
    let person = String::from("David");
    println!("{}", person);
    // we don't need it because it is made automatically
    // but we can invoke drop fn to clean up early
    drop(person);

    let age: i32 = 20;
    println!("{}", age);
    // drop works on stack variables too - it just takes ownership
    drop(age);
}