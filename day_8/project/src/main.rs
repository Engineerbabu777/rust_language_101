fn main() {


    println!("Welcome to rust vector!");

    let vec:Vec<i32> = Vec::new();

    println!("{:?}",vec);

    let mut vec2:Vec<i32> = Vec::new();

    vec2.push(1);
    vec2.push(2);
    vec2.push(3);

    println!("{:?}",vec2);

    let vec3:Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}",vec3);

    let mut vec4:Vec<i32> = vec![1,2,3,4,5];

    vec4.pop();

    println!("{:?}",vec4);


    // reading the elements using get and index!
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    
}
