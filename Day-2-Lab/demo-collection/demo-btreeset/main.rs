use std::collections::BTreeSet;
fn main()
{
let mut shapes = BTreeSet::new();
shapes.insert("square");
shapes.insert("circle");
shapes.insert("triangle");
shapes.insert("rectangle");
shapes.insert("square");
let has_circle = shapes.contains("hexagon");
println!("Do this has circle is HashSet {}",has_circle);

for shape in &shapes{
    println!("Shape is {}",shape);
}

let rm_shape = shapes.remove("circle");
println!("Deleted element is {:?}",rm_shape);
for shape in &shapes{
    println!("Shape is {}",shape);
}

}