// Trait Hierarchy
// How a closure interacts with its captured environment determines which of the three
// closure traits it implements: FnOnce, FnMut, and Fn.
// These traits dictate whether the closure consumes (takes ownership of), mutates (mutably borrows),
// or only reads (immutably borrows) its environment.
// These traits are related through a hierarchy where Fn is the most restrictive
// and FnOnce is the most flexible.

// . Every closure implements FnOnce, as any closure can conceptually be called at least once,
//   potentially consuming its environment in the process.
// . Closures that do not consume their environment (they only borrow it mutably or immutably)
//   also implement FnMut, allowing them to be called multiple times while potentially changing their environment.
// . Closures that only require immutable access to their environment (or capture nothing) also
//   implement Fn, allowing them to be called multiple times without changing their environment.
// This means any closure that implements Fn also satisfies the requirements of FnMut
// and FnOnce, and any closure that implements FnMut also satisfies FnOnce.

// The main difference between them lies in how they capture variables from
// their environment. FnOnce takes ownership of captured values, so it can only
// be called once. This is because once the closure takes ownership of a value,
// it cannot be reused. FnMut borrows captured values mutably, which allows the closure
// to be called multiple times and modify the captured data.
// Fn, on the other hand, borrows captured values immutably, allowing multiple calls
// but no mutation.

// These traits are additive in the sense that a function expecting a closure that
// implements FnOnce can also accept closures that implement FnMut or Fn.
// But if a function expects an Fn, only a closure that implements exactly Fn can be
// passed.

// When you define a function or method that accepts a closure as a parameter,
// you must specify which of these traits it should implement. The choice depends on
// how the closure behaves: whether it moves ownership, mutably borrows, or just reads
// from its environment. Understanding this hierarchy helps write more flexible
// and precise functions in Rust.

// FnOnce: . Closure captures values by move (transferring ownership)
//         . Closure will be invoked once

// FnMut:  . Captures values by mutable reference
//         . Closure can be invoked multiple times

// Fn      . Closure captures values by immutable reference (read-only)
//           or doesn't capture anything at all
//         . Closure can be invoked multiple times

// If a function accepts a FnOnce, you can pass a FnMut or Fn too.
// But if it expects a Fn, only a Fn closure is valid.

// . Choose FnOnce if it needs ownership (and will run only once)
// . Use FnMut if it modifies captured values
// . Use Fn if it just reads data or does nothing with the environment