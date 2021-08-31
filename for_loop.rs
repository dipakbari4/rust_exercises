/**
 * Printing the even numbers
 * in range from 2 to 100
 */
fn main() {
  for i in 2..100 {
    if (i % 2) == 0 {
      print!("{} ", i);
    }
  }
}