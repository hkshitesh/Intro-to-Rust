fn main()
{
    match is_even(10) {
        Some(data) => {
            if data==true{
                println!("Even Number");
            }
        },
        None=> {println!("Not Even");
    }
    }
}

fn is_even(num:i32)->Option<bool>{
    if num%2 == 0{
        Some(true)
    } else{
        None
    }
}