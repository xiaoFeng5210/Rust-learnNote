pub fn test() {
  let mut a = String::from("xiaofeng");
  let a_closure = |x: &mut String| {
    x.push_str(" is a good boy");
    println!("{}", x);
  };
  a_closure(&mut a);
  println!("{}", a);
}
