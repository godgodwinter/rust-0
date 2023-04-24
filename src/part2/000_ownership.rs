fn main() {
    {
        // println!("s value : {s}");
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("s value : {s}");
        // do stuff with s
    }
    // println!("s value : {s}"); // s tidak dapat diakses di luar scope
}
