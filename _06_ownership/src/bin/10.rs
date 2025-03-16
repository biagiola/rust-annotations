// Borrowing means using something without taking ownership of it.
// Most of the time, we're going to be creating references to heap data
// rather from stack, that is also possible but cheaper to worry about.

fn main() {
    // example of borrowring data from the stack
    let my_stack_variable: i32 = 22;
    let my_integer_referece: &i32 = &my_stack_variable;
    // making a reference of a stack value could be the same
    // or ever more than memory required for a integer, that's
    // why we don't usually use references for stack variables

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;

    // References and pointers is almost the same.
    // Reference is a subcategory of pointer.
    // In Rust, a reference is a valid value of a pointer.
    // References must never outlive the referent (the value).
}