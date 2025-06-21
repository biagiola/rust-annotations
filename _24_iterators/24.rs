// Lecture: The filter and find methods V
// More examples using enums and structs

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
    
    // notice that iter() generates references, and filter too, so again we will have &&TVChannels
    // in this case rust will be able to do the dereference by its own on any field or method of the struct
    // without using the asterisc.
    // And about the type of the variable, we can use Vec<_> and the compiler will try to infer, but is better
    // to trying to add by ourself so the compile will tell you any mismatch more properly.
    // let good_channels: Vec<_> = channels.iter().filter(|channel| {
    let good_channels: Vec<&TVChannel> = channels
    .iter()
    .filter(|channel| { channel.channel_type == ChannelType::ProgrammingTutorials })
    .collect();
    
    println!("{good_channels:#?}");
}

// Turbo fish declaration style
// let good_channels = channels
//     .iter()
//     .filter(|channel| { channel.channel_type == ChannelType::ProgrammingTutorials })
//     .collect::<Vec<&TVChannel>>();
