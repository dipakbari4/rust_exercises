/**
 * In this program we'll copy array elements 
 * to another array
 */

fn main() {
  let mut a1 = [4, 65, -8]; // mutable to change elements
  let a2 = [2, 5, -3];      // fixed

  println!("a1 = {:?} ", a1); // printing out
  println!("a2 = {:?} ", a2); // printing out

  a1 = a2;                  // copying array elements
  println!("After copying elements");

  println!("a1 = {:?} ", a1);  // printing out
  println!("a2 = {:?} ", a2);  // printing out
}