// Lecture: Implementing the PartialOrd trait
use std::cmp::Ordering;

#[derive(Eq)]
struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

// impl Eq for Job {} // no need this if we're using in the derive

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary) // u32 implements PartialOrd
    }

    // same as above
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    //     if self.salary == other.salary {
    //         Some(Ordering::Equal)
    //     } else if self.salary < other.salary {
    //         Some(Ordering::Less)
    //     } else if self.salary > other.salary {
    //         Some(Ordering::Greater)
    //     } else {
    //         None
    //     }
    // }
}

fn main() {
    let long_commute_job: Job = Job {
        salary: 100000,
        commute_time: 2,
    };

    let short_commute_job: Job = Job {
        salary: 75000,
        commute_time: 1,
    };

    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job < short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
    println!("{}", long_commute_job >= short_commute_job);
    println!("{}", long_commute_job <= short_commute_job);
}

// Side notes.

// PartialEq gives you == and !=
// . Requires symmetry and transitivity.
// . No promise that a == a is always true (floats break this with NaN).
// . Eq is a marker “child” of PartialEq.
// . Adds the promise of reflexivity (a == a).
// . Many containers (HashMap, HashSet) require keys to be Eq.

// Ordering Enum definition
// pub enum Ordering {
//     Less,    // self < other
//     Equal,   // self == other
//     Greater, // self > other
// }

// PartialOrd trait definition
// pub trait PartialOrd: PartialEq {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering>;

//     // default methods automatically provided once you implement `partial_cmp`
//     fn lt(&self, other: &Self) -> bool { … }
//     fn le(&self, other: &Self) -> bool { … }
//     fn gt(&self, other: &Self) -> bool { … }
//     fn ge(&self, other: &Self) -> bool { … }
// }

// Key points:
// partial_cmp returns Option<Ordering> → it might be None when the two values simply cannot be meaningfully ordered (again NaN for floats, or your own domain-specific reasons).
// Once you implement partial_cmp ONCE, Rust automatically supplies all 6 comparison operators (<, <=, >, >=, and the method aliases above).
