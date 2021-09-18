enum CardinalPoint { North, South, East, West };

let direction = CardinalPoint::South;

match direction {
  CardinalPoint::North => println!("North");
  CardinalPoint::South => println!("South");
}