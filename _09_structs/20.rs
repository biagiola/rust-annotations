#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release: u32,
    duration: u32
}

// In Rust, you are allowed to split the methods and associated functions for a
// single struct into multiple impl blocks.
// The compiler treats all the impl YourStructName blocks as if they were one big
// implementation. Any instance of your struct will have access to all the methods
// defined across all of its impl blocks.
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