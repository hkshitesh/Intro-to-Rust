use std::io;
fn main()
{
    let mut input = String::new();
    println!("enter a number ");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let sub_code:u32 = input.trim().parse().expect("Not entered a number");
    let subject = match sub_code {
        1=>{println!("Found match for CHEM"); "CHEMISTRY"},
        2=> "PHYSICS",
        3=>"Mathematics",
        _=> {println!("Sorry No match Found"); "UNKNOWN"}
    };

    println!("The subject for subject code {} is {}",sub_code,subject);
}