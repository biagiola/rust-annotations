#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release: u32,
    duration: u32
}

impl BobDylanSong {
    // associate function
    // if we not include the self parameter in our fn, it will be
    // an associated one
    fn new(title: String, release: u32, duration: u32) -> Self {
        Self {
            title, release, duration
        }
    }

    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release: {}", self.release);
        println!("Duration: {}", self.duration);
    }
}

fn main() {
    // it looks like a constructor thing built in on the core
    // of rust, but we can change and put any name we want
    let song_0: BobDylanSong = BobDylanSong::new(
        String::from("Blowin' in the wind"),
        1966,
        178
    );
    println!("{:#?}", song_0);
}