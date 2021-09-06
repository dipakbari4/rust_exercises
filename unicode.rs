/**
 * This program will print unicode characters.
 */
fn main() {
  for index in 0..256 {
    println!("{} -> {}", index, index as u8 as char);
  }
}