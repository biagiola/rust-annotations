use std::sync::{Arc, Mutex};
use std::thread;

struct Coffee {
    name: String,
    is_hot: bool,
}

fn main() {
    // 1. Wrap the data in a Mutex, then wrap the Mutex in an Arc.
    let shared_coffee = Arc::new(Mutex::new(Coffee {
        name: String::from("Latte"),
        is_hot: true,
    }));

    let mut handles = vec![];

    for i in 0..2 {
        // 2. Clone the Arc for each new thread. This is cheap.
        let coffee_clone = Arc::clone(&shared_coffee);

        let handle = thread::spawn(move || {
            // 3. Inside the thread, lock the mutex to get mutable access.
            //    The .unwrap() handles a potential error where a thread holding
            //    the lock panics, which "poisons" the mutex.
            let mut coffee_guard = coffee_clone.lock().unwrap();

            println!("Thread {} sees coffee is_hot: {}", i, coffee_guard.is_hot);
            
            // 4. Mutate the data. No other thread can access it now.
            if coffee_guard.is_hot {
                coffee_guard.name = String::from("Iced Latte");
                coffee_guard.is_hot = false;
                println!("Thread {} made the coffee cold.", i);
            }
            // 5. The lock is automatically released when `coffee_guard` goes out of scope.
        });
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Check the final state from the main thread.
    let final_coffee = shared_coffee.lock().unwrap();
    println!("\nFinal coffee state: '{}', is_hot={}", final_coffee.name, final_coffee.is_hot);
}
/*
1. ask about own Tokio async runtimes and how can differ from the standard library.
how can we use the standard library to create a thread pool?

2. also, explain the result logs from the code example.
Thread 1 sees coffee is_hot: true
Thread 1 made the coffee cold.
Thread 0 sees coffee is_hot: false

Final coffee state: 'Iced Latte', is_hot=false

How many tread was created and those threads are in parallel?