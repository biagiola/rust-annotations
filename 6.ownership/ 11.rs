fn main() {
    let my_stack_variable: i32 = 22;
    let my_integer_reference: &i32 = &my_stack_variable;
 
    // print the value of this that is inside of this reference
    println!("{}", *my_integer_reference);
 
    let my_heap_value = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;
    // same for heap data
    println!("{}", *my_heap_reference);
 
    // if we try to print the raw reference it's going to print
    // the value again coz Display trait is focus for human readable
    // so it will be dereference automatically
    println!("{}", my_heap_reference);
    // but :p we get the pointer value
    println!("{:p}", my_heap_reference);
}