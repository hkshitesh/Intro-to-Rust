fn main() { 
    let mut my_string = String::new();
    my_string.push('H');
    my_string.push_str("ello");
    my_string += ", world!";
    println!("{}", my_string);
    let length = my_string.len();
    println!("String length: {}", length);
    let is_empty = my_string.is_empty();
    println!("Is the string empty? {}", is_empty);
    }