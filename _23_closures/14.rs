// Lecture: Fn traits I
// The FN trait is a sub trait of the FN mutant super trait,
// and it represents a closure that captures immutable references
// or doesn't capture any state at all.

// Many of the inline closures that we defined implemented
// the FN trait, but you'll find it to be pretty rare to
// define a closure parameter that mandates the FN trait.
// It is the strictest one of the three because it cannot
// accept an FN once closure or an FN mute closure

// This function takes a generic parameter `F` which must implement the `Fn()` trait.
// That means the passed closure must be callable multiple times *without taking ownership*
// of captured variables (must only capture by reference).
fn execute_thrice<F>(procedure: F)
where
    F: Fn(), // Fn trait: callable multiple times, must not consume captured variables
{
    procedure();
    procedure();
    procedure();
}

fn main() {
    // `bosses` is a vector of string slices (string literals), stored on the heap.
    let bosses = vec!["Boris"];

    // We define a closure that captures `bosses` by **moving** it into the closure.
    // This means the closure takes **ownership** of `bosses`.
    // `bosses` is no longer accessible after this point.
    let closure = || {
        let employees = bosses; // move occurs here
        // `employees` is unused here, it's just for demonstration
    };

    // This call will fail to compile because `closure` takes ownership of `bosses`,
    // and since `execute_thrice` needs to call `closure()` three times,
    // the closure must implement the `Fn` trait.
    //
    // However, closures that take ownership of their environment implement `FnOnce`,
    // not `Fn`, unless they can be called multiple times (which requires no ownership).
    execute_thrice(closure);
}

// You can pass in an FN closure type where a FN once closure is expected,
// but not the inverse. FnOnce is at the very top of the hierarchy,
// and we cannot pass it in to a place that expects FN.

