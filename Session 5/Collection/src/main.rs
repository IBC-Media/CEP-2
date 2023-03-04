// Collections: Vector, HashMap

// we cannot do immutable and mutable reference at the same time....

// .get()   -> return Option type
use std::collections::HashMap;
use std::io;

/*
fn main() {
    let mut scores = HashMap::new();

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("no input");
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut team = scores.get("Blue").copied();   // copied is used to copy the score so that we can change its value later.
    println!("{:?}", team);

    // Updating the value
    scores.insert(String::from("Blue"), 50);
    let t = scores.get("Blue").copied().unwrap_or(0);   // unwrap_or() is used if key is not present then we simply put 0 there.
    println!("{}", t);



    // Accessing the key.

    for (key, value) in &scores {
        println!("{}, {}", key, value)
    }

}

 */

/*
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{}", field_name);
}

 */


// #[allow(dead_code)]
// #[allow(unused_variables)]
// // #[derive(Debug)]
// fn main() {
//     let mut v1 = Vec::<i32>::new();

//     v1.push(10);
//     v1.push(20);
//     v1.push(99);

//     println!("{:?}", v1);

//     // Iterating through the vector

//     for number in v1.iter() {
//         println!("value: {}", number);
//     }

//     // Get value at an index

//     println!("value at index 1: {}", v1[1]);
//     // println!("value at index 100: {}", v1[100]);

//     // println!("value at index 100: {}", v1.get(100));

//     match v1.get(2) {
//         Some(value) => println!("value is: {}", value),
//         None => println!("No such value!!!")
//     }

//     // Creating a vector with a list of values

//     let v2 = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
//     println!("Vector v2 is: {:?}", v2);
//     println!("Vector length: {} Vector capacity: {}", v2.len(), v2.capacity());

// }

/*
#[allow(dead_code)]
// #[derive(Debug)]
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    let mut numerics = HashMap::<i32, String>::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    numerics.insert(1, "One".to_string());
    numerics.insert(2, "Two".to_string());

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let number = 1;
    println!("Number: {} in English is: {}", number, numerics[&number]);
    println!("Does key 5 exist in numerics: {}", numerics.contains_key(&5));
    println!("numerics: {:?}", numerics);
}


 */

/*
// Enum
#[allow(dead_code)]
#[derive(Debug)]
enum Payment {
    Cash,
    Cheque,
    DebitCard(i32),
}

#[allow(dead_code)]
enum AccountType {
    Savings = 10,
    Current = 20,
    FixedDeposit = 30,
}

fn main() {
    let p1 = Payment::Cash;
    let p2 = Payment::DebitCard(500);

    match p1 {
        Payment::Cash => println!("Paying by cash"),
        Payment::Cheque => println!("Paying by Cheque"),
        Payment::DebitCard(amount) => println!("Amount {amount} paid by debit card"),
    }

    match p2 {
        Payment::Cash => println!("Paying by cash"),
        Payment::Cheque => println!("Paying by Cheque"),
        Payment::DebitCard(amount) => println!("Amount {amount} paid by debit card"),
    }

}

 */
// Match operator

// fn main() {

//     // Match Operator
//     // is similar to the 'switch' statement
//     // Uses pattern matching

//     let number = 16;

//     match number {
//         1 => println!("One"),
//         2 => println!("Two"),
//         3 | 5 | 7 | 9 => println!("Odd value less than 10"),
//         10..=20 => println!("Number between 10 and 20"),
//         15..=25 => println!("Number between 15 and 25"),
//         _ => println!("Some other value")
//     };

//     let is_even = match number % 2 {
//         0 => true,
//         1 => false,
//         _ => false
//     };

//     println!("Number: {number} is even: {is_even}");
// }
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {first}");
}