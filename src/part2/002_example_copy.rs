fn main() {
    // !1 .
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);
    // !1. end
    // !2.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // !2. end
}
