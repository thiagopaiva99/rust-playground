enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("we are going up!"),
        Direction::Down => println!("we are going down!"),
        Direction::Left => println!("we are going left!"),
        Direction::Right => println!("we are going right!")
    }
}
