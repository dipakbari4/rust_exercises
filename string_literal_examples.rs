/**
 * This program shows the different style for denoting and the use case
 * of String Literals in Rust Programming.
 */
fn main(){
  let my_number = 21;
  let phrase = "String";
  println!("2");  // Printing out directly
  println!("{}", 2); // Printing an integer using String Literal / Placeholder
  println!("{}", my_number); // printing variable using String Literal / Placeholder
  println!("{} {}!", "Hello", phrase); // Printing out String and var string..
  println!("Statement is {}.", false);  // Passing boolean val..
}