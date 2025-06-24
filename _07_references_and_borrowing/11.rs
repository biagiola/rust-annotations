// 1. A reference stores the memory address where a value is store.
// 2. Borrowing means creating a reference.
// 3. References enable the reuse of data with moving ownership.

// 4. References are immutable by default.
// 5. An immutable reference does not have permission to modify
// the original value at the memory address.
// 6. A value can have any number of immutable references. There is not risk.
// 7. Immutable references implement the copy-trait (full copy).
// Rust will create a full copy in situations where one is needed (variable assignment, function parameters, variable inside array, etc).

// 8. An mutable reference has permission to modify the original value
// at the memory address.
// 9. A value can only have one mutable reference at a time.
// 10. Mutable references do not implement the Copy trait.
// Ownership will move on variable reassignment.
// 11. The compiler understands the reference's lifetime,
// which is the time it is being utilized in the program.
// A lifetime can end before the function's scope

// 12. Composite types like arrays and tuples have ownership over thier elements.
// 13. If an value implements the Copy trait, Rust will create a copy of it when we index into the type.
// 14. If an value does not implement the Copy trait, ownership will move from the compisite type to the new owner.
