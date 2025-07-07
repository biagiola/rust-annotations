// Will this code compile?
// No. The original candy variable is not marked as mutable.

fn main() {
    let candy = String::from("M&M's");
    let some_ref = &mut candy;
    println!("{some_ref}");
}