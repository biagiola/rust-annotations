// This example demonstrates methods that borrow the struct instance,
// either mutably (`&mut self`) or immutably (`&self`).

#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl BobDylanSong {
    // This method takes a mutable reference, allowing it to modify the instance.
    // The `&mut self` syntax is shorthand for `self: &mut Self`.
    fn double_length(&mut self) {
        self.duration_secs *= 2; // multiply and reassigning the new value
    }

    // This method takes an immutable reference, so it can only read data.
    // The `&self` syntax is shorthand for `self: &Self`.
    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }
}

fn main() {
    // we're going to modify one field so we need to make as mut
    let mut song_1: BobDylanSong = BobDylanSong {
        title: String::from("Blowing in the wind"),
        release_year: 1964,
        duration_secs: 210
    };

    let song_2: BobDylanSong = BobDylanSong {
        title: String::from("Like a Rolling Stone"),
        release_year: 1965,
        duration_secs: 194
    };

    if song_1.is_longer_than(&song_2) {
        println!("{} is longer than {}", song_1.title, song_2.title);
    } else {
        println!("{} is longer than {}", song_2.title, song_1.title);
    }

    // We can call `double_length` because `song_1` is mutable.
    println!("Original duration of '{}': {}", song_1.title, song_1.duration_secs);
    song_1.double_length();
    println!("New duration after doubling: {}", song_1.duration_secs);
}
