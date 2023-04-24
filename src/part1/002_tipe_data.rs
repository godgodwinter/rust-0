#![allow(unused)]

use std::io;
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // println!("{guess}")

    // number/integer i
    //u=lebih strict ===
    // number/integer u
    // u= untuk tidak menentukan string/integer misal "32" ==32
    // addition
    let sum = 5 + 10;

    //PECAHAN f
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //BOOLEAN
    let t = true;

    let f: bool = false; // with explicit type annotation

    // COMPOUNTD TYPE
    // multiple value into one type
    // RUSH punya 2 compound : tuples and arrays
    // TUPLE TYPE
    // memasukan beberapa tipe data kedalam 1 type

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // array
    // tidak seperti tuple , setiap elemen memiiki tipe data yang sama. array harus ukuran/length yang fixed.

    // let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5];
    let a = [1, 2, 3, 4, 5];
    // println!("Ini array a = {x}")

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
