pub fn test() {
  let mut arr1 = [1, 2, 3, 4, 5];
  let mut arr2 = &mut arr1[1..3];
  arr2[0] = 6;
  println!("{:?}", arr1);
}
