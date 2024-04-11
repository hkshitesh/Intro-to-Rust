fn main()
{
    let mut fname = String::new();
    println!("Enter a name");
    let b1 = std::io::stdin().read_line(&mut fname).unwrap();
    println!("Hello {}",fname);
    println!("Bytes size in your entered name are {}",b1);
}