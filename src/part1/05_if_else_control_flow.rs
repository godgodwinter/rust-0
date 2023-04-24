fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let numberdua = 3;

    if numberdua == 3 {
        println!("number was three");
    }

    let n3 = 6;

    if n3 % 4 == 0 {
        println!("n3 is divisible by 4");
    } else if n3 % 3 == 0 {
        println!("n3 is divisible by 3");
    } else if n3 % 2 == 0 {
        println!("n3 is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let n4 = if condition { 5 } else { 6 };

    println!("The value of n4 is: {n4}");
}
