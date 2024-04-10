fn main()
{
    hello();
    let mut s1:i32=0;
    s1=sum(40,10);
    println!("Sum is {}",s1);

}
fn hello()
{
    println!("Hello Rust");
}
fn sum(a:i32, b:i32)->i32
{
    let mut s:i32=0;
    s=a+b;
    // println!("Sum is {}",s);
    return s;
}