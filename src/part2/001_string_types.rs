fn main() {
    {
        {
            let mut s = String::from("hello");
            // do stuff with s
            s.push_str(", world!"); // push_str() appends a literal to a String

            println!("{}", s); // This will print `hello, world!`
        }
        // println!("{}", s); // s tidak dapat diakses
    }
    // println!("s value : {s}"); // s tidak dapat diakses di luar scope
}
