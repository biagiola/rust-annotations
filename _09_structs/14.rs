#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

// A method is a function that belongs to a instance.
// Those fn's are declare separate from its values.
impl BobDylanSong {
    // Self is the alias for it will be like write BobDylanSong struct
    // fn display_song(self: Self) { // and we can short it even more
    // for a type 'immutable struct value'
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} secs", self.duration_secs);
    }
}

fn main() {
    let song: BobDylanSong = BobDylanSong {
        title: String::from("Like a rolling stone"),
        release_year: 1961,
        duration_secs: 194
    };

    song.display_song_info();
}

// Method receivers in `impl` blocks determine ownership and mutability.
// There are four common forms:
// 1. `self`: Takes ownership of the struct instance (moves it).
// 2. `mut self`: Takes ownership and allows mutation.
// 3. `&self`: Borrows the instance immutably.
// 4. `&mut self`: Borrows the instance mutably.