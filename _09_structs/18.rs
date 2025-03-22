#[derive(Debug)]

struct BobDylanSong {
    title: String,
    release_year: u32,
    duration: u32
}

impl BobDylanSong {
    fn display_song_info(&self) {
        println!("{}", self.title);
        println!("{}", self.release_year);
        println!("{}", self.duration);
    }

    fn double_length(&mut self) {
        self.duration = self.duration * 2;
    }

    fn years_since_release(&self) {
        let a: u32 = 2025 - self.release_year;
        println!("{a}");
    }
}

fn main () {
    let mut song_1: BobDylanSong = BobDylanSong {
        title: String::from("Blowing in the wind"),
        release_year: 1965,
        duration: 230
    };

    // song_1.double_length();
    // song_1.display_song_info();
    song_1.years_since_release();

}