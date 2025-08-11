// Lecture: The filter and find methods VI
// From the previous lecture, let say we want a vector of the names that
// we have on the channels vector.

#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials
}

#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType
}

fn main() {
    let channels: [TVChannel; 4] = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News
        },
        TVChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials
        },
    ];
    
    // 1. Without using references, also without taking ownership
    let good_channels: Vec<String> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials )
        .map(|channel| channel.name.clone() ) // we need to use clone to avoid take ownership of this heap data
        .collect();
    println!("{good_channels:?}");

    // 2. Using references, also without taking ownership
    // When you borrow references you dont have to worry about ownership but you have to deal with the complexity of references.
    // So, it's always trade offs.
    let good_channels: Option<&TVChannel> = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials); // find the first

    match good_channels {
        Some(channel) => println!("Great choise to watch {channel:?}"),
        None => println!("There was no Rust programming on the tv")
    };
}
