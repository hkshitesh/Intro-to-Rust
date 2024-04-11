fn main()
{
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(30);
    numbers.push(50);
    let mut v_len = numbers.len();
    println!("Length of Number vector is {}",v_len);
    println!("Vector elements are {:?}",numbers);
    let first_no = numbers[0];
    let sec_no = numbers[1];
    println!("First num is {} and second num is {}",first_no,sec_no);
    let del_element= numbers.remove(0);
    println!("Vector elements are {:?}",numbers);
    println!("deleted element is {}",del_element);    
    v_len = numbers.len();
    println!("Length of Number vector is {}",v_len);    
}