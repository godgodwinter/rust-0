// !ex1
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

//! ex2
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// !ex2 fixed
// fn main() {
//     let mut s = String::from("hello");
//     println!("{s}");
//     change(&mut s);
//     println!("{s}")
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
//! larangan
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);
// !larangan lain
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }

// !alternatif
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }
// !Dangling References
// !error
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!
// !solusi
fn main() {
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}")
}
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
