// #![allow(unused_variables)]
// #![allow(dead_code)]
// #[derive(Debug)]
//
// // Traits - Define shared behaviour for multiple structs
// //          They are similar to interfaces
//
//
// struct Boeing {
//     model: String,
//     range: i32
// }
//
// #[derive(Debug)]
// struct Airbus {
//     model: String,
//     range: i32
// }
//
// trait Flight {
//     fn valid_flight(&self, distance: i32) -> bool;
// }
//
// impl Flight for Boeing {
//     fn valid_flight(&self, distance: i32) -> bool {
//         self.range + 300 > distance
//     }
// }
//
// impl Flight for Airbus {
//     fn valid_flight(&self, distance: i32) -> bool {
//         self.range + 500 > distance
//     }
// }
//
//
// fn main() {
//     let boeing = Boeing {
//         model: "747".to_string(),
//         range: 1500
//     };
//
//     let airbus = Airbus {
//         model: "A320".to_string(),
//         range: 2000
//     };
//
//     println!("boeing: {:?}", boeing);
//     println!("boeing flight for 1000km is valid: {}", boeing.valid_flight(1000));
//
//     println!("boeing: {:?}", airbus);
//     println!("boeing flight for 2600km is valid: {}", airbus.valid_flight(2600));
//
// }

/*
enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn get_days(days: i32) {
    match days {
        1 => println!("{}",Days::Monday),
        2 => println!("{}", Days::Tuesday),
        3 => println!("{}",Days::Wednesday),
        4 => println!("{}", Days::Thursday),
        5 => println!("{}",Days::Friday),
        6 => println!("{}", Days::Saturday),
        7 => println!("{}",Days::Sunday),
    }
}

#[derive(Debug)]
struct p {
    name: i32,
    fp: i32,
}



 */

