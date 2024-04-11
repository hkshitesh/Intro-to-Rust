// macro_rules! add {
//     ($a:expr, $b:expr)=>{
//         $a+$b
//     };
//     ($a:expr)=>{
//         $a
//     }
// }
// fn add_fn(a:i32,b:i32)->i32
// {
//     a+b
// }

// fn main()
// {
//     let sum1 = add!(2,3);
//     let sum2 = add_fn(10,20);
//     let x = add!(10);    
//     println!("Sum is {}",sum1);
//     println!("Sum is {}",sum2);
//     println!("X is {}",x);
// }

// macro_rules! unit_converter {
//     ($fn_name:ident, $conversion:expr) =>{
//         fn $fn_name(val:f64) -> f64 {
//             val*$conversion
//         }
//     };
// }

// unit_converter!(inches_to_cm,2.54);
// unit_converter!(km_to_m,1000.0);
// fn main()
// {
//     let inches = 10.0;
//     let cm = inches_to_cm(inches);
//     println!("{} inches is {} centimeters",inches,cm);
//     let km =25.5;
//     let m = km_to_m(km);
//     println!("{} Km is {} meters",km,m);
// }

macro_rules! repeat {
    ($a:expr, $times:expr)=>{
        for _ in 0..$times {
            $a
        }
    };
}

fn main()
{
    repeat!(println!("Hello Rust"),100);
}