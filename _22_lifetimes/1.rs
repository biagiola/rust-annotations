// Lecture: The lifetime of a value refers to how long it is calid within the code.
// A value's lifetime is the tiem during which it exists at a particular memory address.

fn main() {
    let a = 1;

    {
        let b = 2;
    }

    let c = String::from("Winter");
    let d = c;
}

// lifetime and scope almost always are very tight to each other
// but of course we can have a lifetime end before the end
// of its scope, for example when we passing ownership.

fn main() {
    let a = 1;

    {
        let b = 2;
    }

    let c = String::from("Winter");
    drop(c);
}

// also, we can say that the lifetime of c will end when we 
// pass it as argument to a function.
// Its scope is still the main function but the lifetime is
// not to the end of it.