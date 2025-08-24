use std::collections::HashMap;

fn main() {
    let mut playlist = HashMap::new();
    
    // FnMut: captures playlist by mutable reference
    let add_song = |song: String, genre: String| {
        playlist.insert(song, genre); // Mutably borrows playlist
    };
    
    // its ownership going to be moved, but we don't care
    let bohemian_rhapsody = String::from("Bohemian Rhapsody");
    let shape_of_you = String::from("Shape of you");
    
    // add song can be called multiple times - that's why it's FnMut, not FnOnce
    add_song(bohemian_rhapsody, "Rock".to_string());
    add_song(shape_of_you, "Pop".to_string());
    
    println!("{:?}", playlist);
}

// calling `add_song` requires mutable binding due to mutable borrow of `playlist`
