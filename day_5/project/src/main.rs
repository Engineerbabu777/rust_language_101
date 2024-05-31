
pub mod methods;

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: &str,
//     sign_in_count: u64,
// }
// fn build_user(email: &str, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

struct Color (i32,String,bool);


fn main() {
    println!("Hello to Structs!");

  
//   let user1 = build_user("string",String::from("@gmai;.com"));
  let black = Color(0, String::from("something"), false);

  methods::Methods::Test();
//   let user2 = User {
//     active: user1.active,
//     username: user1.username,
//     email: "another@example.com",
//     sign_in_count: user1.sign_in_count,
// };

//   println!("{:?}", user2);
  println!("{:?}", black.0);


}
