
#[derive(Debug)]
#[allow(dead_code)]
struct User<T,V>{
   x:T,
   y:V,
   //z:U
}

fn main() {
    println!("Structs Generics!");

    // create an instance of user!
    let user1 = User{
     x:56,
     y:'c',     
    };


    // print the instance!
    println!("{:?}", user1);


}
