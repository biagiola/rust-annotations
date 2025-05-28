// Trait Hierarchy
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