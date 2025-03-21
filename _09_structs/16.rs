// self
// mut self
// &self
// &mut self
#[derive(Debug)]
 
struct BobDylanSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl BobDylanSong {
    // -> Immutable ref to the struct instance (no ownership moved)
    // fn display_song_info(self: &BobDylanSong) {
    // fn display_song_info(self: &Self) {
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} secs", self.duration_secs);
    }

    // -> Mutable ref to the struct instance (no ownerhsip moved, has permission to mutate)
    // fn double_length(self: &mut BobDylanSong) {
    // fn double_length(self: &mut Self) {
    fn double_length(&mut self) {
        // self.duration_secs = self.duration_secs * 2; // short version
        self.duration_secs = (*self).duration_secs * 2;
        println!("{:#?}", self);
    }
}

fn main() {
    let mut song: BobDylanSong = BobDylanSong {
        title: String::from("Like a rolling stone"),
        release_year: 1965,
        duration_secs: 201
    };
    song.display_song_info();
    song.double_length(); // &song // rust do this behind the scenes
}