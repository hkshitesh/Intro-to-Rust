

use std::collections::HashMap;

fn main() {

let mut contacts = HashMap::new();

    loop {
        println!("Enter name and phone number (e.g. John +1234567890), or type q to quit:");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim() == "q" {
            break;
        }
        
        let tokens: Vec<&str> = input.trim().split("+").collect();
        
        if tokens.len() != 2 {
            println!("Invalid input. Please try again.");
            continue;
        }
        
        let name = tokens[0].trim().to_string();
        let phone_number = tokens[1].trim().to_string();
        
        contacts.insert(name.clone(), phone_number.clone());
        
        println!("Added contact: {} {}", name, phone_number);
    }


    println!("Contacts:");
    for (name, phone_number) in contacts.iter() {
        println!("{}: {}", name, phone_number);
    }
}