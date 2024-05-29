

// using the std lib!
// use std::io;

// enum Condition {
//     PASS,
//     FAIL,
//     OK
// }

#[allow(dead_code)]
pub mod conditions {

    pub fn test() {
        println!("Hello, world!");
    
        another_function();
    }
    
    pub fn another_function() {
        println!("Another function.");
    }

    pub fn loop_over_array() {
        let a = [10, 20, 30, 40, 50];
    
        for element in a {
            println!("the value is: {element}");
        }
    }
}