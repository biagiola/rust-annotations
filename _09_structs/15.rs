// Method receivers in `impl` blocks determine ownership and mutability.
// There are four common forms:
// 1. `self`: Takes ownership of the struct instance (moves it).
// 2. `mut self`: Takes ownership and allows mutation.
// 3. `&self`: Borrows the instance immutably.
// 4. `&mut self`: Borrows the instance mutably.
//
// This example focuses on `self` and `mut self`, which move ownership.

#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl BobDylanSong {
    // This method takes ownership of the instance via `self`.
    // The instance is moved into the method and consumed.
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
    }

    // This method takes ownership and can mutate the instance via `mut self`.
    // The instance is also moved and consumed here.
    fn double_length(mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }
}

fn main() {
    let song: BobDylanSong = BobDylanSong {
        title: String::from("Like a Rolling Stone"),
        release_year: 1965,
        duration_secs: 194
    };

    // The `display_song_info` method takes ownership of `song` via the `self`
    // parameter. This "consumes" the struct instance.
    song.display_song_info();

    // Because `song` was moved (consumed) by the call above, it cannot be used
    // again. The line below would cause a compile-time error.
    // song.double_length(); // error[E0382]: use of moved value: `song`
}
