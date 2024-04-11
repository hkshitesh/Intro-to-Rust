use std::collections::HashMap;
fn main()
{
    let mut marks = HashMap::new();
    marks.insert("Alice",95);
    marks.insert("Bob",78);
    marks.insert("Tom",89);
    marks.insert("Kerry",100);
    let Alice_marks = marks.get("Alice");
    let has_marks = marks.contains_key("Dalton");
    println!("Alice marks are {:?}",Alice_marks);
    println!("Do we have Bob Marks {:?}",has_marks);
    for (key, val) in & marks{
        println!("Name is {}: Marks are {}",key, val);
    }
}