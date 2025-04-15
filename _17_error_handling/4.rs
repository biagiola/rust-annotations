use std::process;

fn main() {
    println!("Some status update");
    eprintln!("Some error message");
    // e stands for errors that prints message to what's called the standard error
    // that's in comparision to println! which prints message to what's called stadard output

    // eprintln macro is equivalente to the println, except that the output goes to
    // io::stderr rather the io::stdout.
}

// we can see one main difference when we're writting files in the terminal
// using the cargo command plus the redirection operator from the linux system.
// redirection operation basically, it is use to redirects the output of the given
// program from the terminal to something else.
// >> cargo run > file.txt

// so the println message will not see it in the console when we run the program,
// but it will be attached in file.txt, and if we have an error, we will want to
// show to the user the message error right into the console and not pasted into
// the file. To use another channel for that, and that's is the stderr stream
// or channel with the help of eprintln macro. 