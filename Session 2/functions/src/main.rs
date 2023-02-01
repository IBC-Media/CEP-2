/*

fn main() {
    /*
    let n = 99;
    println!("Value of n is {}", n);
    n += 10; // we cannot change the value of n because we have defined n as immutable.


     */

    // Defining Mutable variable
    let mut n =  99; // we are defining n as mutable variable.
    println!("Value of n is {}", n);
    n += 10;
    println!("new value of n is {}", n);

    // Defining const
    const PI: f32 = 3.14;   // name of const variable should be in capital letter. and also we need type annotation.

}

 */

// All about functions
// Function => A named block of code that return a value.
// Procedure => A named block of code that does some work but does not return a value.

fn main()
{
    welcome();
    welcome_user("john Doe");
    println!("test function returns {}", test());
    println!("even_odd_fun returns {}", even_or_odd(12));
    let username = String::from("abcd");
    println!("username is {}", username);
    println!("username in capitals {}", captalize(username));
    // println!("After calling capitalize function, username is {}", username);   this line will give error after uncomment. because the ownership of string is transfered.
}

// Function that does not return a value (aka procedure)
fn welcome() {
    println!("Welcome to Rust programming.");
}

// &str -> string slice
// Function that takes in arguments and does not return value (aka procedure)
fn welcome_user(name: &str) {
    println!("Welcome {}", name);
}

// Function that returns a value
fn test() -> i32{
    5
}

fn even_or_odd(n: i32) -> String {
    if n% 2 == 0 {
        String::from("Even")
    }
    else {
        String::from("Odd")
    }
}
fn captalize(s: String) -> String {
    s.to_uppercase()
}







