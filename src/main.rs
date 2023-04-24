fn main() {
    // note:
    // 1. varibale immutable atau tidak bisa di reassign
    // 2. bisa diubah menggunakan aritmatika fungsi
    // ----
    // IMMUTABLE / cant reassign /muted variable concept
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    //CONSTANTA
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // THREE_HOURS_IN_SECONDS=22;
    println!("Constanta {THREE_HOURS_IN_SECONDS}");
    // SHADOWING varibale
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        // x=4;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let mut spaces = "   ";
    // spaces = spaces.len();
}
