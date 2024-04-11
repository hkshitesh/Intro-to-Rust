// fn main()
// {
//     let cmd_ip = std::env::args();
//     println!("No. of arguments passed {}",cmd_ip.len());    

//     for arg in cmd_ip{
//         println!("arguments are {}",arg);
//     }
// }

fn main()
{
    let cmd_ip = std::env::args();
    println!("No. of arguments passed {}",cmd_ip.len());   

    let mut sum =0;
    let mut has_read_first_arg=false;
    
    for arg in cmd_ip{
        if has_read_first_arg{
            sum = sum + arg.parse::<i32>().unwrap();
        }else{
            has_read_first_arg=true;
        }        
    }
    println!("Sum is {}",sum);
}
