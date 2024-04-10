 // Ownership transfer

// fn main()
// {
//     let x1 = vec![2,4,6];
//     println!(" x1 is before x2=x1 {:?}",x1);
//     let x2 = x1;
//     // println!(" x1 is after x2=x1 {:?}",x1);
//     println!(" x2 is before calling display {:?}",x2);
//     let x2_return=display(x2);
//     println!(" x2_return is  after calling display {:?}",x2_return);
// }

// fn display(v:Vec<i32>)->Vec<i32> {
//     println!("inside display {:?}",v);
//     v
// }

// Borrowing concept 

fn main()
{
    let x1 = vec![2,4,6];
    println!(" x1 is before x2=x1 {:?}",x1);
    let x2 = x1;
    // println!(" x1 is after x2=x1 {:?}",x1);
    println!(" x2 is before calling display {:?}",x2);
    display(&x2);
    println!(" x2 is  after calling display {:?}",x2);
}

fn display(v:&Vec<i32>) {
    println!("inside display {:?}",v);    
}