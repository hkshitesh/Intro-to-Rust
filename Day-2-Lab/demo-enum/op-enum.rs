fn main()
{
    let res = is_even(11);
    println!("{:?}", res);
    println!("{:?}", is_even(24));
}

fn is_even(num:i32)->Option<bool>{
    if num%2 == 0{
        Some(true)
    } else{
        None
    }
}