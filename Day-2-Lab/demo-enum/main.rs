#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn print_dir(dir: Direction) {
    match dir {
        Direction::Up => println!("Moving UP"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right")
    }
}

fn main()
{
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    print_dir(up);
    print_dir(down);
    print_dir(left);
    print_dir(right);

    // println!("My Current Direction is {:?}",up);
    // println!("My Current Direction is {:?}",down);
    // println!("My Current Direction is {:?}",left);
    // println!("My Current Direction is {:?}",right);
}