// Structures

// Declaring a struct
// struct Student {
//     name: String,
//     rollno: i32,
//     cgpa: f32
// }

// fn main() {
// let student = ("abcd", 101, 3.5);

// Instantiating a struct

//     let mut st1 = Student {
//         name: "ABCD".to_string(),
//         rollno: 101,
//         cgpa: 3.5
//     };

    // println!("st1 is {:?}", st1);

//     // println!("Name of student: {}", st1.name);
//     show_student_name(&st1);
//     increment_rollno(&mut st1);

//     st1.cgpa = 3.9;
//     println!("st1 is {:?}", st1);


//     let st2 = create_student("XYZ".to_string(), 123, 3.3);
//     println!("st2 is {:?}", st2);

//     // Let's create another student with same score as st3

//     // Method - 1: Explicit values
//     // let st3 = Student {
//     //     name: "Dummy".to_string(),
//     //     rollno: 555,
//     //     cgpa: 3.3
//     // };

//     // // Method - 2: Copy data from st2
//     // let st3 = Student {
//     //     name: "Dummy".to_string(),
//     //     rollno: 555,
//     //     cgpa: st2.cgpa
//     // };


//     // Method - 3: Copy all data from st2 and customize the required fields
//     let st3 = Student {
//         name: "Dummy".to_string(),
//         rollno: 555,
//         ..st2
//     };

//     println!("st3 is {:?}", st3);

// }

// fn show_student_name(s: &Student) {
//     println!("Name: {}", s.name);
// }

// fn increment_rollno(s: &mut Student) {
//     s.rollno += 1;
// }

// fn create_student(name: String, rollno: i32, score: f32) -> Student {
//     // Student { 
//     //     name: name, 
//     //     rollno: rollno, 
//     //     cgpa: score
//     // }

//     Student { 
//         name, 
//         rollno, 
//         cgpa: score
//     }

// }


// Methods: Structure implementation blocks

// struct Student {
//     name: String,
//     rollno: i32,
//     cgpa: f32
// }

// impl Student {
//     fn is_passed(&self) -> bool {
//         self.cgpa > 1.5
//     }

//     fn lowercase_student_name(&self) -> Self {
//         Self {
//             name: self.name.to_lowercase(),
//             ..*self
//         }
//     }
// }

// fn main() {
//     let st1 = Student {
//         name: "ABCD".to_string(),
//         rollno: 101,
//         cgpa: 0.5
//     };
//
//
//     println!("The student has passed: {}", st1.is_passed());
//
//     let st2 = st1.lowercase_student_name();
//     println!("st2 is {:?}", st2);
//
// }

