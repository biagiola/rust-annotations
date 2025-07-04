// Borrowing means using something without taking ownership of it.
// Most of the time, we create references to heap data rather than stack data.
// Referencing stack data is possible but often unnecessary due to memory costs.

fn main() {
    // example of borrowing data from the stack
    let my_stack_variable: i32 = 22;
    let my_integer_reference: &i32 = &my_stack_variable;
    // making a reference of a stack value could use the same
    // or even more memory than the integer itself, that's
    // why we don't usually use references for stack variables

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;

    // References are a type of safe pointer in Rust.
    // Unlike raw pointers, references are always valid and safe to use.
    // References must never outlive the referent (the value they point to).
}