fn main() {
    let time: i32 = 2025;
    let year: i32 = time;
    // here, coz we're using integers, we make a copy trait.
    // Rust create a full copy of the integer value on the stack

    let person: String = String::from("David Biagiola");
    let user: String = person;
    // For 'user', we're not using copy trait. Rust copy the
    // reference of the heap data, the length and capacity
    // from the person stack entry to a new 'user' stack entry.
    // There is sill one piece of heap data, but now we have
    // two references of it on the stack. 'user' become the owner,
    // which means we cannot use person after line 8.
    // 'user' is responsable for deallocating the string's heap
    // memory when user varaible goes out of the scope.

    // | OWNER     - VALUE |
    // | reference - x556b | <- 'person' deallocated (not longer exists really)
    // | length    - 14    | 
    // | capacity  - 20    |
    // |-------------------|
    // | reference - x556b | <- 'user' takes ownership
    // | length    - 14    |
    // | capacity  - 20    |  | David Biagiola |
    // |------ STACK ------|  |----- HEAP -----|

    // In others languages, without the concept of ownership
    // there could be two references poiting to the same heap data
    // so, when the fn scope would end, the program would remove the 'person' variable,
    // it would try to remove the contents on the heap.
    // But that program would also have the 'user' variable going out of scope,
    // and that would attempt to clear the location and memory that was already cleared
    // by the previous variable going out of scope. So, when the language tries to
    // free availabe memory on the heap twice, it actually runs into an error,
    // coz the heap alloc will technically no longer have a value. This error is called
    // double free error. It can lead to memory corruption and security vulnerabilities,
    // but it's not possible with Rust.
    // The compiler error is something like:
    // "Error: Borrow of moved value 'person': moves occurs because 'person' has type 'String'
    // which does not implement the Copy trait"
}
