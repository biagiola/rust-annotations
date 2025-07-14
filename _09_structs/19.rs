// This example explains the difference between methods and associated functions.
// It also introduces the `Self` keyword and the `new` constructor pattern.

#[derive(Debug)]
struct BobDylanSong {
    title: String,
    release: u32,
    duration: u32,
}

impl BobDylanSong {
    // This is an ASSOCIATED FUNCTION.
    // It does not take `self` as a parameter and is associated with the
    // `BobDylanSong` type itself, not an instance.
    // By convention, `new` is used as a constructor to create new instances.
    //
    // `-> Self` is a return type declaration, where `Self` is an alias
    // for the type in the `impl` block, in this case `BobDylanSong`.
    fn new(title: String, release: u32, duration: u32) -> Self {
        // `Self { ... }` creates and returns a new instance of BobDylanSong.
        Self {
            title, release, duration
        }
    }

    // This is a METHOD.
    // It takes `&self` as a parameter, meaning it operates on a specific,
    // borrowed instance of `BobDylanSong`.
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release: {}", self.release);
        println!("Duration: {}s", self.duration);
    }
}

fn main() {
    // We call the associated function `new` on the `BobDylanSong` type
    // itself, using the `::` syntax. This is like a static method call.
    let song_0: BobDylanSong = BobDylanSong::new(
        String::from("Blowin' in the Wind"),
        1963,
        168,
    );

    let song_1: BobDylanSong = BobDylanSong::new(
        String::from("Like a Rolling Stone"),
        1964,
        203,
    );

    // We then call the `display_song_info` method on our new `song_0`
    // instance, using the `.` syntax.
    song_0.display_song_info();
    song_1.display_song_info();
}

// 1. Associated Functions vs. Methods
// Methods: These take a form of self (self, &self, or &mut self) as their first parameter.
// They are called on a specific instance of a struct, using a dot (.).
// Example: song_0.display_song_info() calls the display_song_info method on the song_0 instance.
//
// Associated Functions: These do not take self as their first parameter. They are associated with
// the type itself, not a specific instance.
// They are called using the type's name and a double colon (::).
// This is very similar to "static methods" in other languages.
// Example: BobDylanSong::new(...) calls the new function that belongs to the BobDylanSong type.