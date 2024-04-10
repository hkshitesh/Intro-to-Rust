fn main()
{
    let mut arr:[i32;5] = [5,7,9,11,13];
    // let arr = [5,7,9,11,13];
    // let arr:[i32;5]= [-1;5];
    println!("Array elements are {:?}",arr);
    println!("Array size is {}",arr.len());

    arr[1]=45;


    // for i in 0..5{
    //     println!("Array at {} index is {}",i,arr[i]);
    // }

    // for val in arr.iter(){
    //     println!("Array elements are {}",val);

    // }

    println!("Array element at 3rd index is {}", arr[3]);
}