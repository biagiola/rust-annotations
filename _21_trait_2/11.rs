// Lecture: Implement the Clone trait
// this give us the ability to create a duplicate of an instance

use std::clone::Clone; 

// #[derive(Clone)] // if we add this we'll have the clone method like the declare
// but every field must to implement the clone trait itself.
// #[derive(Clone, Debug)] // and of course, you can also implement more that one
// traits for example the clone and the debug at the same time.
#[derive(Debug)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// our implementation, could be use if one of the filed do not implement the clone method
// and we want to declare how they will copy
impl Clone for Appointment {
    fn clone(&self) -> Self { // we need to return the same fields for our copy
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

fn main() {
    let morning_appt: Appointment = Appointment::new("Dr. Andrews", "11:45AM", "12:30AM");
    let replacement_appt =  morning_appt.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_appt.doctor, replacement_appt.start_time, replacement_appt.end_time
    );

    // morning_appt still exist
    println!("{morning_appt:?}");
}

// remainder: we're accepting &str in our doctor just to be more flexible
// and have the ability to recept: &str and &String