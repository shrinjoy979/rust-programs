enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    move_around("north".to_string());
}

fn move_around(direction: String) {
    if direction == "north" {
        println!("Moving North");
    }
}