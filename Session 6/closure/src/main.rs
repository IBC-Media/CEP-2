// fn main() {
//     println!("Hello, world!");
//     println!("Indian currency symbol is: \u{20b9} ☻");

//     let hello = String::from("नमस्ते");
//     // let s = &hello[0..4];
//     // println!("s is {}", s);

//     print!("[");
//     for x in hello.as_bytes() {
//         print!("{}, ", x);
//     }
//     println!("]");

//     print!("[");
//     for x in hello.chars() {
//         print!("{}, ", x);
//     }
//     println!("]");

//     // print!("[");
//     // for x in hello. {
//     //     print!("{}, ", x);
//     // }
//     // println!("]");

//     let b = true;

//     match b {
//         true => "True",
//         false => "False"
//     };

//     #[allow(dead_code)]
//     enum States {
//         TS,
//         TN,
//         KA,
//         DL,
//         UP,
//         MP,
//     }

//     let indian_state = States::UP;

//     let southern_state = match indian_state {
//         States::TS => "Telangana",
//         States::TN => "Tamilnadu",
//         States::KA => "Karnataka",
//         _ => "Not a southern state"
//     };

//     println!("southern_state: {}", southern_state);

// }

// const VAL:u8 = 99;
#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let n = 55;

    fn test() {
        println!("This is the test function");
        // println!("Value of n is {}", n);
    }

    // (|| {
    //     println!("This is the closure");
    //     println!("Value of n is: {}", n);
    // })();

    let my_closure = || {
        println!("This is the closure");
        println!("Value of n is: {}", n);
    };

    test();
    my_closure();

    // Closure with parameters and return values

    let clos_ret_val = || -> u32 { 123456 };

    println!("Calling clos_ret-val return the value: {}", clos_ret_val());

    let clos_add = |first: u32, second: u32| -> u32 {
        let sum = first + second;
        sum
    };

    println!(
        "Sum of {} and {} using closure is {}",
        10,
        15,
        clos_add(10, 15)
    );

    // Power of functional programming using closures
    // Functional programming => Functions are treated as first-class objects
    //                           Therefore, functions can be passed to other functions
    //                           as arguments!!
    //                           In Rust this can be done using closures.

    let add = |a: u32, b: u32| -> u32 { a + b };

    let subtract = |a: u32, b: u32| -> u32 { a - b };

    let compute = |operation: fn(u32, u32) -> u32, a: u32, b: u32| -> u32 { operation(a, b) };

    let n1 = 50;
    let n2 = 20;
    // println!("add gives: {}", add(n1, n2));
    println!("USING FUNTIONAL PROGRAMMING");
    println!("passing add into compute");
    println!(
        "Using add with compute the result is: {}",
        compute(add, n1, n2)
    );

    println!("passing subtract into compute");
    println!(
        "Using subtract with compute the result is: {}",
        compute(subtract, n1, n2)
    );
}
