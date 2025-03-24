// The match kayword II
enum OperatingSystem {
    Windows,
    MacOS,
    Linux
}

fn main() {
    let my_computer: OperatingSystem = OperatingSystem::MacOS;
    let age: u32 = years_since_release(my_computer);
    println!("{result}");

    // just a recap on ownership rules, my_computer variable is
    // transfer its owner to the year_since_release fn
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Windows is a very old system");
            39
        },
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34
    }
    // here, we need to cover each possible enum arm
    // and also, we have to use curly braces for code blocks
}