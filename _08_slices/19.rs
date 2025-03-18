fn main () {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let slice_of_three: &[i32] = &values[..3];
    
    print_length(slice_of_three); // type mismatch
}
 
fn print_length(reference: &[i32; 6]) { // but if we do not specify the length it would be ok
    println!("{}", reference.len());
}