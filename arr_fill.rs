/**
 * This program demonstrates the use of array fill
 * and the fixed specified size
 */

 fn main() {

  let stock = [2; 10];  // the array filled with value 2 at fixed length 10

  for item in stock {  
    println!("{}", item); // printing values
  }
 }