#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release: u32,
    duration: u32
}

// we can have multiple implementations, and make merge together
// here is a dummy example just for know it, but we dive deep more in later lectures
impl BobDylanSong {
    // in this case we use the associated fn but we could use any other
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs
        }
    }
}
    
impl BobDylanSong {
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} secs", self.duration_secs);
    }
}
    
fn main() {
    ...
}