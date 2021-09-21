/**
 * In this program we'll see about the
 * functions with arguments
 **/

fn main() {
  print!("{}", add(3.14, 5.5));
}

fn add(param1: f32, param2: f32)-> f32 {
  return param1 + param2;
}