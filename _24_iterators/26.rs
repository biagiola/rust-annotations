// Lecture: The any and all methods

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
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    // All method, under the hood
    let good_channels: Vec<_> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .collect();
    println!("{}", good_channels.len() == channels.len());

    // All method, better aproach
    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{all_are_rust}");

    // Any method, under the hood
    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{}", good_channel.is_some());

    // Any method, better aproach 
    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{any_are_rust}");
    
}