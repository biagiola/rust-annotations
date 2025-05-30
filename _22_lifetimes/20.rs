// Lecture: Lifetime Elision Rules II
struct DentisAppointment {
  doctor: String
}

// what happens when we don't return a value from the self, so we need to avoid
// third elision rule, so in that case we need to provide lifetimes annotations.
// impl DentisAppointment {
//   // in this case the borrow checker is inferring about the return value from self
//   fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
//     println!(
//       "You are booked from {} to {} with doctor {}",
//       check_in_time, check_out_time, self.doctor
//     );
//     check_in_time
//   }
// }

// full example
// impl DentisAppointment {
//   fn book<'a, 'b, 'c>(&'a self, check_in_time: &'a str, check_out_time: &'c str) -> &'c str {
//     println!(
//       "You are booked from {} to {} with doctor {}",
//       check_in_time, check_out_time, self.doctor
//     );
//     check_out_time
//   }
// }

// shorter example
impl DentisAppointment {
  fn book<'a>(&self, check_in_time: &str, check_out_time: &'a str) -> &'a str {
    println!(
      "You are booked from {} to {} with doctor {}",
      check_in_time, check_out_time, self.doctor
    );
    check_out_time
  }
  // The most important thing here is by using one generci lifetime 'a, I've created the
  // marker or the arrow pointing from the parameter to the returned value, so now the 
  // borrow checker knows that the returned reference must live within the lifetime of the
  // referent from wichi 'check_out_time' came.
}

fn main() {
  let appt = DentisAppointment {
    doctor: String::from("David"),
  };
  let result: &str = appt.book("03:00PM", "04:00PM");
  drop(appt); // now it doesn't matter if we drop it
  println!("{result}");

}
