use std::fmt::format;
use std::io;

// LOOPS

// loop  => Infinite loop
// while  => Conditional loop
// for   => Iterator loop


/*
// Let's generate multiplication table with different loops.
fn main() {

    // using loops statement
    let mut n = 10;
    let mut counter = 1;
    loop {
        println!("{n} * {counter} = {}", n * counter);
        counter += 1;
        if counter >= 11 {
            break;
        }

    }

    // Using while loop

    counter = 1;  //again assign counter = 1;
    while counter <= 10 {
        println!("{n} * {counter} = {}", n * counter);
        counter += 1;
    }
/*
    // Using for loop

    println!("Enter a number for multiplication: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");   // If we face any error to read the input then .expect will return the error.
    n = input.trim().parse().expect("Error parsing"); //
    // counter = 1;
    for i in 1..11 {   // 11 is exclusive
        println!("{n} * {i} = {}", n * i);
    }

 */
}


 */

use std::mem;

fn main() {
    /// hjh
    let mut b = 10;
    let a = "a".to_string();
    let b = "b".to_string();
    let c = "c".to_string();
    let x = format!("{}{}{}", a,b,c);
    // println!("{}{}", r1, r2);
}