#![allow(unused_variables)]

/*
fn main() {
    let mut n = 5;
    println!("Before calling increment_v1: n is {}", n);
    n = increment_v1(n);
    println!("After calling increment_v1: n is {}", n);

    println!("Before calling increment_v2: n is {}", n);
    increment_v2(&mut n);
    println!("After calling increment_v2: n is {}", n);

}

// Returns the incremented value
fn increment_v1(x: i32) -> i32 {
    x + 1
}

// This function increments the variable in-place
fn increment_v2(x: &mut i32) {
    *x = *x + 1;
}

 */

// Move
// fn main() {
//     let s1 = String::from("hello");
//     println!("s1 is {}", s1);
//
//     let s2 = s1; // The ownership of string is MOVED to s2
//                          // so s1 is no longer valid!!
//
//     println!("s1 is {}", s1); // s1 is invalidated due to move!!
//     println!("s2 is {}", s2);
//
// }

/*
fn main() {
    let s1 = String::from("hello");
    println!("s1 is {}", s1);
    display_string(s1);
    println!("After calling display_string s1 is {}", s1);  // Won't work due to move
                                                            // in the function call to
                                                            // display_string
}

fn display_string(s: String) {
    println!("Your string is {}", s);
}

 */

/*
fn main() {
    let s1 = String::from("hello");
    println!("s1 is {}", s1);
    display_string(&s1);
    println!("After calling display_string s1 is {}", s1);
}

fn display_string(s: &String) {
    println!("Your string is {}", s);
}


 */

// String

/*
fn main() {
    let mut s1: String = String::from("hello");

    println!("s1 is {}", s1);
    add_world(&mut s1);
    println!("After calling add_world s1 is {}", s1);
}

fn add_world(s: &mut String) {
    s.push_str(" World!!");
}


 */
//

struct student {
    first_name: String,
    second: String,
}

fn main()
{
    let mut name = student {
         first_name: String::from("Deepak"),
        second: String::from("chaudhary"),
    };


}
