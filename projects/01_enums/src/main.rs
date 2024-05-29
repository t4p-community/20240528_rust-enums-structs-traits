enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}
