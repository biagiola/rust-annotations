#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    disk: u32
}

impl Computer {
    fn new(cpu: String, memory: u32, disk: u32) -> Self {
        Self {
            cpu, memory, disk
        }
    }

    fn upgrade_cpu(&mut self, cpu: String) -> &mut Self {
        self.cpu = cpu;
        self
    }
    fn upgrade_memory(&mut self, memory: u32) -> &mut Self {
        self.memory = memory;
        self
    }
    fn upgrade_disk(&mut self, disk: u32) -> &mut Self {
        self.disk = disk;
        self
    }
}

fn main() {
    let mut computer: Computer = Computer::new(String::from("M3 Max"), 64, 2);
    // computer.upgrade_cpu("M4 Max");
    // computer.upgrade_memory(128);
    // computer.upgrade_disk(4);

    // building pattern
    computer.upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_disk(4);

    println!("Stats: {computer:#?}");
}