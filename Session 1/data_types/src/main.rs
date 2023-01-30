
use std::io;   // for taking user input

fn main() {
    let _n = 55;  // Variable definition without type information, compiler will automatically infer
    // the type based on the value assigned to the variable
    let _x: u8 = 45;  // variable definition with "Type annotation". Because we are not using x in future so we should use '_x' to avoid the warnings.
    println!("Hello, world!");

    // Define Bool variable
    let bool_var = true;
    println!("Value of bool_var is {}", bool_var);

    let hex_val = 0xff32;
    let oct_val = 0o2471;
    let bin_val = 0b1100_0011;
    let byt_val = b'A';

    println!("hex {} oct {} bin {} byte {}", hex_val, oct_val, bin_val,byt_val);

    // Tuple
    // It is a collection of heterogeneous types

    let tup = (10,20,5.67,false);
    println!("tup is {:?}", tup);

    // Creating a tuple with type annotation
    let t2: (i32, bool, f32, i32) = (1, true,10.,1);
    // println!("t2 is {:?}", t2);

    println!("First element {}, second element {}, third element {}, forth element {}", t2.0, t2.1, t2.2, t2.3);

    // Arrays.
    // It is a collection of homogeneous types
    let a: [i32;5] = [1,2,3,4,5];
    println!("a {:?}", a);
    println!("a[0]: {}, a[1]: {}", a[0], a[1]);

    // Taking user input
    let a = [1, 2, 3, 4, 5];

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
