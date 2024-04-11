use std::collections::VecDeque;

fn main()
{
    let mut vdeque: VecDeque<i32> = VecDeque::new();
    vdeque.push_back(10);
    vdeque.push_back(30);
    vdeque.push_back(40);
    vdeque.push_front(20);
    vdeque.push_front(15);
    println!("VDeques element are {:?}",vdeque);
    println!("front element is {:?}", vdeque.front());
    println!("back element is {:?}", vdeque.back());

    let el_front = vdeque.pop_front();
    let el_back = vdeque.pop_back();

    println!("VDeques element are {:?}",vdeque);
    println!("front deleted element is {:?}", el_front);
    println!("back deleted element is {:?}", el_back);
}