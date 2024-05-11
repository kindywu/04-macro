use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(dead_code)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down(i32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    // let up = Direction::Up(DirectionUp::new(42));
    let up: Direction<i32> = DirectionUp::new(42).into();
    println!("{:?}", up);
    let down: Direction<i32> = 42.into();
    println!("{:?}", down)
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

// impl From<DirectionUp> for Direction {
//     fn from(value: DirectionUp) -> Self {
//         Direction::Up(value)
//     }
// }
