// Lecture: FnOnce traits closures

fn main() {
    let number: i32 = 13;
    let capture_number = || number;
    println!("{number}"); // numbers is still available

    let a: i32 = capture_number();
    let b: i32 = capture_number();
    println!("{a} {b} {number}"); // no moves of ownership here

    // but with heap data we move ownership
    let first_name = String::from("Alice");
    let capture_string = || first_name;
    // println!("{first_name}"); // side note: the movement of ownership happens
    // right away after closure declaration, not in the invocation. So...
    // first_name is not the owner of 'Alice' string anymore.

    let new_owner = capture_string();
    
    // another side note: movement of ownership is a one time operation. We cannot
    // repeat the same process to the sames variables again coz after a variables
    // is dangling, is deleted by the rust compiler. Same principle for closure with
    // an FnOnce.
}