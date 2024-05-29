
pub mod cond;

#[allow(unused_variables)]

fn main() {

    // VARIABLES IN RUST!
    let x = 5;
    println!("The Value of mutable {x}");
    // BY DEFAULT ALL THE VARIABLES ARE IMMUTABLE!

    // WE CAN MAKE THEM MUTABLE BY ADDING MUT KEYWORD AFTER THE LET KEYWORD!!
    let mut x = 5;

    println!("The Value of x before being changed! {x}");
    x = 6;

    // CONSTANTS!
    const ABC:i32 = 56;
    println!("The Value of constant ABC is: {ABC}");
    // ALSO NEED TO KEEP THIS THING IN MIND THAT CONSTANTS ARE IMMUTABLE AND ALSO WE HAVE TO DEFINE THE TYPE COMPILER WONT SUGGEST THE TYPE AT COMPILE TIME!

    println!("The Value of {x}");

    // WE CAN SHOWED THE VARIABLE IN THE SAME SCOPE AS WELL!

    // STRING METHOD TO GET THE LENGTH OF THE STRING!
    let y:&str = "The is a string literal!";
    let length:usize = y.len();
    println!("The length of the string literal stored on the stacked is {length}");

     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
     let truncated = -5 / 3; // Results in -1
 
     // remainder
     let remainder = 43 % 5;


     // COMPOUNDS DATA TYPES!
     // allow debug traits
     let tuples:(i32,char,bool) = (33,'C',true);
     println!("The tuples are: {:?}",tuples);

     cond::conditions::test();
     cond::conditions::loop_over_array();

}
