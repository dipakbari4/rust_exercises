/**
 * In this we'll make a program in which
 * 'struct' keyword will use
 * It's similar to c/c++ version
 *
 */

 fn main() {
   struct SomeData {
     integer: i32,
     fractional: f32,
     character: char,
     five_bytes: [u8; 5],
   }

   
   let data = SomeData {
     integer: 10_000_000,
     fractional: 183.19,
     character: 'Q',
     five_bytes: [9, 0, 250, 60, 200],
   };


   println!("{} {} {} {} ", data.integer, data.fractional, data.character, data.five_bytes[2]);
 }