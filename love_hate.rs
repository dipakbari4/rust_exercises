fn main() {
  let love = true;    // Love is truth
  let hate = ! love;  // Hate is not

  if hate > love {    
    // If hate is greater than love...
    println!("I hate programming!");
  
  } else if love > hate { 
    // Or if love is greater...
    println!("I love programming!");
  
  } else {
    
    println!("Let me think again!");
  }
}
