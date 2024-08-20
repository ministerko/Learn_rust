fn main() {
  #[derive(Debug)]
  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@gmail.com"),
    sign_in_count: 1,
  };
  user1.email = String::from("anotheremail@example.com");
  println!("{:?}", user1);
}