use std::io::{self, Write};

fn main() {
    let mut names = Vec::new();

    loop {
        print!("Enter name (type 'q' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let name = input.trim();

        if name == "q" {
            break;
        }

        names.push(name.to_string());
    }

    println!("List of entered names: {:?}", names);
}