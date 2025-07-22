use std::process;

fn main() {
    process::exit(0);
    // finish the execution with zero which means we don't have any errors.
    println!("This will not print");
}

// Commonly used exit codes on Unix/Linux
// 0 – Success (everything went fine)
// 1 – Generic/unspecified error
// 2 – Misuse of shell built-ins (bad command-line usage in many CLI tools)
// 126 – Command found but not executable
// 127 – Command not found
// 128 – Invalid argument to exit (128 + n is also used for signals; see below)
// 130 – Script terminated by Ctrl-C (128 + SIGINT (2) = 130)
// 137 – Killed by SIGKILL (128 + 9) – often “out-of-memory killer”
// 139 – Segmentation fault (128 + 11)
// 143 – Terminated by SIGTERM (128 + 15)

// General rules
// 0 → success
// 1–125 → application-defined errors
// 126–165 → command could not start or was killed by a signal (128 + signal number)
// 255 (or −1) → “catch-all” in some programs, but safe to avoid because shells often truncate to 8 bits.