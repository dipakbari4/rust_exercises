/**
 * This program demonstrates the use of function
 * and calling from inside main function
 * 
*/

fn main() {
  println!("Adding two vars");
  add(3, 5);
}

fn add(num1: i32, num2: i32) {
  println!("{}", num1 + num2);
}