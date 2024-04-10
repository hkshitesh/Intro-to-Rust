// use std::io;
// fn main() {
//     println!("Hello, world!");
//     let emoji:char = '😁';
//     println!("emoji is {}",emoji);
//     let f2 = 12;
//     println!("f2 is {}",f2);

//     let stdin = 10;

//     let mut input_text = String::new();
//     io::stdin()
//         .read_line(&mut input_text)
//         .expect("failed to read from stdin");

//     let trimmed:u32 = input_text.trim().parse().expect("Not a number");
//     match trimmed {

//     // match stdin {
//         1 => println!("Sunday!"),
//         2 => println!("Monday!"),
//         3 => println!("Tuesday!"),
//         4 => println!("Wednesday!"),
//         5 => println!("Thursday!"),
//         6 => println!("Friday!"),
//         7 => println!("Saturday!"),
//         _ => println!("Invalid input")
//     }

// }

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    return (temp * 1.8) + 32.0;
}

fn main() {
    let celsius_temp = 30.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("{}°C is equal to {}°F", celsius_temp, fahrenheit_temp);
}
