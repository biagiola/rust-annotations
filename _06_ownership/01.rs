// Memory
// . Ownership is a set of rules that the compiler checks for to ensure the program will be free of memory errors.
// . Memory refers to the area of your computer that is responsible for storing the information your programs use.
// . it's ideal to free memory when it is no longer in use.
// . Programming languages implement different strategies for memory management.

// . Languages like java, python, ruby and go implement a tool called the garbage collector.
// . The garbage collector looks for data that is no longer in use and deallocates it. It "automates" the cleanup process.
// . The garbage collector itself occupies memory and can run at disadvantageous times.

// Rust introduces a new paradigm: ownership.
// . Ownership is set of rules on how Rust manages your computer's memory.
// . The rust compiler does not compile the program if an ownership rule is violeted.
// . Best of all worlds: the speed of a language like C but with less room for error.

// . The owner is who/what is responsible for cleaning up a piece of data when it is no longer in use.
// . Every value in rust program has one owner.
// . The owner can change over the course of the program, but there is only 1 owner for a value at a time.
// . The owner is usually a name: A variable can be an owner. A parameter can be an owner.
// . Ownership also extends to composite types that own thier elements. A tuple and array onw thier values.

// The Stack and the Heap
// . The stack and the heap are two different parts/regions of the computer's memory.
// . The stack and the heap read and write data in different ways that offer advantages and disadvantages

// The stack (LIFO) is generally fast, but it only supports data of a fixed, predictable constant size and also, that size must be known at compile time. The heap is generally slower, but it supports dynamic data that can change in size over the program's execution.
// Data types like integers, floating-points, booleans, characters and arrays have a fixed size. Rust stores them on the stack at runtime. The pieces of data on the stack will not grow or shrink in size as the program runs.

// . The heap is a large area of storage space. Think of it like a warehouse.
// . The heap is for data whose size is not known at compile time (user input, a file's content, etc)
// . When the rust program needs dynamic space, it request it from the heap. A program called the memory allocator finds an empty spot that is large enough to store the data.

// References:
// . The memory allocator returns a reference, which is an address.
// . the reference points to the memory address of the data.
// . Think of a parking lot giving you a reference (spot "h25") when they park your car.
// . We can store a reference in a variable in a rust program. References have a fixed size, so Rust stores them on the stack.

// The Heap II
// . Allocating on the heap is slower than pushing to the stack. The memory allocator has to spend time searching for an open spot large enought to fit the data.
// . Accesing data is faster on the stack than the heap as well. With a heap, the program has to follow the pointer to find the memory address.
// . A stack stores the data in sequence, so there is less "jumping around" from point to point.

// . The porpuse of ownership is to assign responsibility for deallocating memory (primary heap memory)
// . Ownership is a compiler feature for reducing duplicate heap data and cleaning up heap data that is no longer needed.