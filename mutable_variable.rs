// In this program we'll learn about
// mutable variable. Means a kind of
// variable that allows us to make
// changes to their values.
// Let's see how it works.

fn main() {   // traditional main function starts


  let mut my_var = 23;  // 'mut' keyword allows us to make mutable variable. :-)

  // printing out my_var variable value
  println!("my_var: {}", my_var);

  // Adding 14 to current value of my_var variable
  my_var = my_var + 14; 

  println!("my_var: {}", my_var);

}