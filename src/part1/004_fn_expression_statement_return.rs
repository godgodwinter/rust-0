// note:
// 1 . return tidak perlu ;
fn five() -> i32 {
    5 + 4
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
    //Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    let y = {
        let x = 3;
        x + 1
        // This expression:
        // let x = 3;
        // x + 1
    };

    println!("The value of y is: {y}");

    let abc = plus_one(5);

    println!("The value of x is: {abc}");
}

// fn plus_one(x: i32) -> i32 {
//     x + 1; // ; tidak mengembalikan nilai /void
// }

fn plus_one(x: i32) -> i32 {
    x + 1 //tidak perlu titik ;
}

// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
