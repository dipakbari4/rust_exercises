/**
 * In this program we'll cover basics
 * on enumeration and switch case usage.
 */


fn main() {
  enum Continent {
    Europe,
    Asia,
    Africa,
    America,
    Oceania
  }
  
  let contin = Continent::Asia;
  
  match contin {
    Continent::Europe => print!("E"),
    Continent::Asia => print!("As"),
    Continent::Africa => print!("Af"),
    Continent::America => print!("Am"),
    Continent::Oceania => print!("Oc"),
  }
}