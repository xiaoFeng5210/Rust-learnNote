pub fn test_owner_ship() {
  let x = String::from("hello world!");
  owner_ship_print(x);
}

fn owner_ship_print(x: String) {
  println!("{}", x);
}
