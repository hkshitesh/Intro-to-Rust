use std::thread;
// use std::time::Duration;

// fn main()
// {
//     let handle= thread::spawn(|| {
//         for i in 1..10{
//             println!("number {} from spawn thread ",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5{
//         println!("number {} from main thread ",i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap();
// }

//Example 2

fn sum_range(start:usize, end: usize) -> usize {
    let mut sum =0;
    for num in start..=end {
        sum = sum+num;
    }
    return sum;
}

fn main()
{
    const N:usize= 100000;
    const THREADS: usize =2;
    let mut handles = vec![];
    for i in 0..THREADS {
        let start = i*(N/THREADS as usize)+1;
        let end = (i+1)*(N/THREADS as usize);
        let handle = thread::spawn(move|| sum_range(start,end));
        handles.push(handle);
    }
    let mut final_sum=0;
    for handle in handles{
       // println!("{:?}",handle.join().unwrap());
        final_sum= final_sum+handle.join().unwrap();
    }
    println!("Sum of numbers from 1 to {} is {}",N, final_sum);
}