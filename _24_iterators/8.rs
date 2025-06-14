// Lecture: HashMap iteration

use std::collections::HashMap;

fn main() {
    let mut todos: HashMap<&str, bool> = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    // OWNERSHIP
    // for (todo, completion_status) in todos { // todo: &str, completion_status: bool
    //     println!("Task: {todo}. Complete: {completion_status}");
    // }
    // println!("{todos:?}"); // we loose ownership

    // IMMUTABLE REFERENCES
    // for (todo, completion_status) in &todos { // todo: &&str, completion_status: &bool
    //     println!("Task: {todo}. Complete: {completion_status}");
    // }
    // println!("{todos:?}"); // now it keep its ownershop

    // MUTABLE REFERENCES
    // for (todo, completion_status) in &mut todos { // todo: &&str, completion_status: &mut bool
    for (_, completion_status) in &mut todos { // completion_status: &mut bool
        // println!("Complete {completion_status}");
        // string slices are not modifiable to that's is an exception for the mutation rule
        // but for the boolean is okay. So to avoid any future errors we just name it with
        // the underscode name.
        // Also this will the case where we'll need the dereferance operator. The compiler
        // accidentally thinks that we are trying to override a reference variable with a
        // new boolean value. What re really mean to say is go to the memory address where
        // we're getting and mutate it. We have permission to do so due the &mut.
        *completion_status = true;
    }
    println!("{todos:?}");
}