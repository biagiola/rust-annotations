fn main() {
    let mut music_genres: &str = "  Rock, Metal, Country, Rap  ";
    println!("{}", music_genres.trim()); // no aditional data is created
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("Metal", "Funk"));

    // splits delimiter
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:#?}", genres);

}