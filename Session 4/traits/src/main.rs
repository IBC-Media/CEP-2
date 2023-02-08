#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Debug)]

// Traits - Define shared behaviour for multiple structs
//          They are similar to interfaces


struct Boeing {
    model: String,
    range: i32
}

#[derive(Debug)]
struct Airbus {
    model: String,
    range: i32
}

trait Flight {
    fn valid_flight(&self, distance: i32) -> bool;
}

impl Flight for Boeing {
    fn valid_flight(&self, distance: i32) -> bool {
        self.range + 300 > distance
    }
}

impl Flight for Airbus {
    fn valid_flight(&self, distance: i32) -> bool {
        self.range + 500 > distance
    }
}


fn main() {
    let boeing = Boeing {
        model: "747".to_string(),
        range: 1500
    };

    let airbus = Airbus {
        model: "A320".to_string(),
        range: 2000
    };

    println!("boeing: {:?}", boeing);
    println!("boeing flight for 1000km is valid: {}", boeing.valid_flight(1000));

    println!("boeing: {:?}", airbus);
    println!("boeing flight for 2600km is valid: {}", airbus.valid_flight(2600));

}