use macros::EnumFromDarling;

#[derive(Debug, EnumFromDarling)]
#[allow(dead_code)]
enum Direction {
    Up(DirectionUp),
    Down(i32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

fn main() {
    // let up = Direction::Up(DirectionUp::new(42));
    let up: Direction = DirectionUp::new(42).into();
    println!("{:?}", up);
    let down: Direction = 42.into();
    println!("{:?}", down)
}

impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}

// impl From<DirectionUp> for Direction {
//     fn from(value: DirectionUp) -> Self {
//         Direction::Up(value)
//     }
// }
