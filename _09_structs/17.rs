#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl BobDylanSong {
    fn double_length(&mut self) {
        self.duration_secs * 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }
}

fn main() {
    let song_1: BobDylanSong = BobDylanSong {
        title: String::from("Blowing in the wind"),
        release_year: 1964,
        duration_secs: 210
    };

    let song_2: BobDylanSong = BobDylanSong {
        title: String::from("Like a rolling stones"),
        release_year: 1965,
        duration_secs: 194
    };

    if song_1.is_longer_than(&song_2) {
        println!("{} is longer than {}", song_1.title, song_2.title);
    } else {
        println!("{} is longer than {}", song_2.title, song_1.title);
    }
}