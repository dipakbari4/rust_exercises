/**
 * This program shows the call of 'len' function to get 
 * total array elements and access it through
 * enhanced for loop
 */
fn main() {
  let fruits = ["apple", "banana", "guave", "kiwi", "raspberry"];

  println!("total fruits: {}" , fruits.len());

  for fruit in fruits {
    println!("{}", fruit);
  }
}