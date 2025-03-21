#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl BobDylanSong {
    // Mutable struct value (self param takes ownership, has permission to mutate)
    fn double_length(mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }
    // fn double_length() {
    //     println!("test");
    // }

    // Immutable struct value (self param takes ownership)
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
    }
}

fn main() {
    let song: BobDylanSong = BobDylanSong {
        title: String::from("Like arolling stone"),
        release_year: 1965,
        duration_secs: 194
    };
    song.display_song_info();
    // Here display_song_info method takes ownership of
    // the BobDylanSong self variable, consuming the struct.
    // So, we cannot use another method that use the same
    // struct again, like double_lenth. Even if we want to
    // use just for reading purpose.
    // song.double_length(); // don't work if we use self on it.
    // Examples about it using references it will use in 3 and 4 examples.

    // Now, just as a side note. If we change double_length
    // to not use the self, I mean, BobDylanSong struct, and
    // make just a simple println, we're now getting not 
    // a method but a associated function. So, we should
    // use another syntax.
    // BobDylanSong::double_length();
}

