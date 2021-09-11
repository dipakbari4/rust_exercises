/**
 * This program will print unicode characters.
 */
fn main() {
  for index in 0..256 {
    if(index%5==0) {
      println!("{} -> {}", index, index as u8 as char);
    } else {
      print!("{} -> {}\t", index, index as u8 as char);
    }
  }
}