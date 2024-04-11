// fn main()
// {
//     let arr = [20,40,60];
//     println!("{}",arr[4]);
//     println!("End line of code");
// }

// fn main() {
//     let no = 13; 
//     //try with odd and even
//     if no%2 == 0 {
//        println!("Thank you , number is even");
//     } else {
//        panic!("NOT_AN_EVEN"); 
//     }
//     println!("End of main");
//  }
 

// use std::fs::File;
// fn main() {
//    let f = File::open("abc.txt"); 
//    //this file does not exist
//    println!("{:?}",f);
//    println!("End of main");
// }

// use std::fs::File;
// fn main() {
//    let f = File::open("abc.txt");   // abc.txt doesn't exist
//    match f {
//       Ok(f)=> {
//          println!("file found {:?}",f);
//       },
//       Err(e)=> {
//          println!("file not found \n{:?}",e);   //handled error
//       }
//    }
//    println!("end of main");
// }



fn main(){
    let result = is_even(13);
    match result {
       Ok(d)=>{
          println!("no is even {}",d);
       },
       Err(msg)=>{
          println!("Error msg is {}",msg);
       }
    }
    println!("end of main");
 }
 fn is_even(num:i32)->Result<bool,String> {
    if num%2==0 {
       return Ok(true);
    } else {
       return Err("NOT_AN_EVEN".to_string());
    }
 }
 
