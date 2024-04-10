
fn main()
{
let mut n:i32 =5;
println!("value of n before calling convert_to_zero is {}",n);
// convert_to_zero(n);
// println!("value of n in main is {}",n);
mut_convert_to_zero(&mut n);
println!("value of n in main is {}",n);
}

//call by value

fn convert_to_zero(mut num:i32){
    num = 0;
    println!("value of num in fucntion is {}",num);
}

//call by ref
fn mut_convert_to_zero(num:&mut i32)
{
    *num=0;
    println!("value of num in fucntion is {}",*num);
}
