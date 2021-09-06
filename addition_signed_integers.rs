fn main() {
  let a: i8 = 5;
  let b: i16 = 5;

  // This will generate a compilation
  // error due to mismatched archs
  print!("{}", a + b);
}