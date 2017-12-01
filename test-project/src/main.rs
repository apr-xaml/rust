pub enum Direction { North, South, East, West }

pub fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}

fn main() {
    let two = 1+1;
    println!("{}", two);

    let points = Direction::South;
    let compass = is_north(points);
    println!("{}", compass);
}
