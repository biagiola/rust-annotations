use std::collections::HashMap;

fn main() {
    let first_value = String::from("David");
    let second_value = String::from("Biagiola");

    // let playlist: HashMap<&String, &String> = HashMap::from([
    //     (&first_value, &second_value),
    // ]);

    // Note: playlist variable does not have mutability
    let playlist: HashMap<String, String> = HashMap::from([
        (first_value, second_value),
    ]);
    
    // FnOnce: takes ownership of playlist from environment
    let consume_playlist = || {
        let owned_playlist = playlist; // <-- playlist is MOVED into the closure
        println!("Consumed: {:?}", owned_playlist);
        // playlist is consumed here - can't call this closure again
    };
    
    consume_playlist(); // Works
    // consume_playlist(); // ❌ Won't compile - playlist already consumed

    // println!("{first_value}");
    // println!("{second_value}");
}

// FnMut implies the closure can be called multiple times as long as it keeps
// the necessary mutable borrow(s) alive between calls.
// Moving playlist destroys the original binding, so the very contract of
// “callable multiple times” is broken. Hence FnMut is invalid.

// How could we make it FnMut or Fn?
// let mut playlist = HashMap::<String, String>::new();
// let mut add_song = |song: String, genre: String| {
//     playlist.insert(song, genre);   // uses &mut playlist
// };
// `add_song` is FnMut

