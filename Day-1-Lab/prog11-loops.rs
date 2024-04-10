fn main()
{
    //using for loop
    // for i in 1..11{
    //     println!("i is {}",i)
    // }


    //using while loop
    // let mut i=1;
    // while i<11{
    //     println!("i is {}",i);
    //     i=i+1;
    // }

    //using Loop

    // let mut i=1;
    // loop{
    //     println!("i is {}",i);
    //     i=i+1;
    //     if i==11{
    //         break;
    //     }
    // }

    //using for loop with continue
    for i in 1..11{
        if i%2==0{
        println!("i is {}",i)
        }
        else{
            continue;
        }
    }
}