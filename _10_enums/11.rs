// The match Keyword V - match with exact value
enum Milk {
    Lowfat(i32),
    Whole
}

impl Milk {
    // just a side note, we're going to transfer ownership
    // but in this case doesn't care us.
    fn drink(self) {
        match self {
            // for this match, is not only the Lowfat, but also with the argument of 2
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favorite!")
            }
            Milk::Lowfat(percent) => { // all the others values caome after
                println!("You've got the lowfat {percent}%")
            }
            Milk::Whole => {
                println!("You've got the whole milk!")
            }
        }
    }
}

fn main() {
    Milk::Lowfat(1).drink();
    Milk::Lowfat(2).drink();
    Milk::Whole.drink();
}