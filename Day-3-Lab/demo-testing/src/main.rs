mod tests;

fn factorial(n:u32)->u32{
    if n==0  {
        return 1;
    }
    else {
        return n*factorial(n-1);
    }
}

fn add(a:i32, b:i32)->i32
{
    return a*b;
}

fn main()
{
    let n1=5;
    println!("Factorial of {} is {}",n1,factorial(n1));
    let n2 =6;
    println!("Factorial of {} is {}",n2,factorial(n2));
    let n3 =7;
    println!("Factorial of {} is {}",n3,factorial(n3));
    println!("Sum is {}",add(5,4));
}