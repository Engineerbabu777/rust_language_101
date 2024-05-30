

// Implement the !
#[allow(dead_code)]

// CREATE A FUNCTION!
fn scope(){
    let x:&str = "Hello";

    println!("The value of the {x}");
} // THE VALUE (x) WILL BE DROPPED HERE!

#[allow(dead_code)]
fn string(){
    let string = String::from("Hello");
    println!("The value of the variable stored on the stack is: {string}");
}

// FOR MUTATING STRING!
#[allow(dead_code)]
fn mutating_string(){
    let mut string = String::from("well");
    string.push_str("Why 🙄?");

    println!("{string}");
}

#[allow(dead_code)]
fn giving_ownership(){
    let x = String::from("Hello");
    let y = x;


    // AS THE VALUE X IS BEING borrow SO WE CANNOT USE IT FURHTER MORE!
    // println!("{0},{1}",x,y);
    println!("{}",y);

}

#[allow(dead_code)]
fn shallow_deep_copy(){
    let x = String::from("Hello from shallow and deep copy fn!");
    let y = x;

    println!("The value of {y}");
}

#[allow(dead_code)]
fn borrowing_and_reference(s:&str){

    println!("The referenced value: {s}");
}


fn first_word(s: &String) -> usize {
    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    for (i,item) in s.chars().enumerate() {
        if item == ' ' {
            return i;
        }   
    }

    s.len()
}
fn main(){

    let x = String::from("Hello this is borrowed value!");
    // scope();
    // string();
    // giving_ownership();
    // mutating_string();
    // shallow_deep_copy()
    borrowing_and_reference(&x);
    let len = first_word(&x);
    println!("{}",len);

}