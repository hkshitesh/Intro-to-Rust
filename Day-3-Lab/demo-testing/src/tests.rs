use crate::factorial;
use crate::add;

#[test]
fn test_factorial()
{
    assert_eq!(factorial(5),120);
    assert_eq!(factorial(4),24);
    assert_eq!(factorial(1),1);
    assert_eq!(factorial(0),1);
}

#[test]
fn test_add()
{
    assert_eq!(add(2,3),5);
    assert_eq!(add(10,20),30);
}