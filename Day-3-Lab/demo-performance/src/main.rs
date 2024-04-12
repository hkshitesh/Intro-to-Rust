use rayon::prelude::*;
use std::time::Instant;

fn sum_of_square_seq(start:u64, end:u64) ->u64 {
    let mut sum:u64 =0;
    for num in start..=end{
        sum = sum + num*num as u64;
    }
    return sum;
}

fn sum_of_square_parallel(start:u64, end:u64) ->u64{
    (start..=end).into_par_iter().map(|num| (num*num) as u64).sum()
}

fn main()
{
    let start = 1;
    let end =1000000;

    let start_time = Instant::now();
    let sum_seq = sum_of_square_seq(start,end);
    let elapsed_time_seq = start_time.elapsed();

    let start_time = Instant::now();
    let sum_parallel = sum_of_square_parallel(start,end);
    let elapsed_time_parallel = start_time.elapsed();

    println!("Result of Sum from sum_seq method is {} ",sum_seq);
    println!("Result of Sum from sum_parallel method is {} ",sum_parallel);

    println!("Time taken by sum_seq is {:?}", elapsed_time_seq);
    println!("Time taken by sum_parallel is {:?}", elapsed_time_parallel);


}
