fn main() {
    let mut names = Vec::new();    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim() == "q" {
            break;
        }
        
        names.push(input.trim().to_string());
    }
    
    println!("The list of names: {:?}", names);
}