use std::collections::HashMap;

fn main() {
    let my_vector = vec![4, 8, 12, 16, 23, 42];
    let my_iterator = my_vector.into_iter(); // its type is IntoIter<i32> and we're taking ownership
    // println!("{my_vector:?}"); // so we cannot use the variable anymore

    let my_vector = vec![false, true, false];
    let my_iterator = my_vector.into_iter(); // its type is IntoIter<bool>
    
    let mut my_hashmap: HashMap<&str, i32> = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator = my_hashmap.into_iter(); // its type is IntoIter<&str, i32>

    // println!("{my_hashmap:?}"); // so we cannot use the variable anymore
    println!("{my_iterator:?}");
}
