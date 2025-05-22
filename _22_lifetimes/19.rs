// Lecture: Lifetime Elision Rules II
// These are rules that the borrow checker applies when it evaluates references.
// If the rules are sufficient to establish lifetime associations, leave out explicit
// lifetime annotations because the borrow checker is able to figure them out.

// . First rule is that every reference in a parameter list gets a lifetime,
// . Second rule is that if the return value is a reference and there is only one
// reference parameter, the borrow checker will associate the lifetimes of
// the two together.

// Third Elision Rule: In a method definition, if there are multiple reference params
// but one of them is self, the borrow checker will assume the lifetime of the instance
// is connected to the lifetime of the return value.
//
struct DentisAppointment {
  doctor: String
}

impl DentisAppointment {
  // this is the third rule infered by the borrow checker.
  fn book<'a, 'b, 'c>(&'a self, check_in_time: &'b str, check_out_time: &'c str) -> &'a str {
    println!(
      "You are booked from {} to {} with doctor {}",
      check_in_time, check_out_time, self.doctor
    );
    &self.doctor // this is s String but thanks to deref coersion we cast to &str
  }
}
fn main() {
  let appt = DentisAppointment {
    doctor: String::from("David"),
  };
  let result: &str = appt.book("03:00PM", "04:00PM");
  // drop(appt); // if we add this line it will not compile. Move out of 'appt' and a borrow happens later
  println!("{result}");

}

// so thanks to the third rule we can make shorter fn signature like this:
// fn book(&self, check_in_time: &str, check_out_time: & str) -> & str { ... }