#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl BobDylanSong {
    // Immutable struct value (self param takes ownership)
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
    }

    // Mutable struct value (self param takes ownership, has permission to mutate)
    fn double_length(mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }
}

fn main() {
    let song: BobDylanSong = BobDylanSong {
        title: String::from("Like arolling stone"),
        release_year: 1965,
        duration_secs: 194
    };
    song.display_song_info();
    // btw we can call the method above if we want to call
    // another method again coz we loose ownership for song
    // variable that was passed to self. If we call it from
    // the first method call and no longer exists after a
    // second method use or call.
    // song.double_length();
}

