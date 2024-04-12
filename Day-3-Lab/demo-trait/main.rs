trait Shape
{
    fn area(&self)->f64;
}

struct Rect
{
    width:f64,
    height:f64,
}

impl Shape for Rect
{
    fn area(&self)->f64
    {
        self.width*self.height
    }
}
struct Circle
{
    radius:f64,
}

impl Shape for Circle
{
    fn area(&self)->f64
    {
        3.14*self.radius*self.radius
    }
}

fn main()
{
    let rect = Rect{
        width:10.0,
        height: 8.0,
    };
    let circle = Circle{
        radius:10.0,
    };
    let rect_area= rect.area();
    let cir_area = circle.area();
    println!("Area of Rectangle is {}",rect_area);
    println!("Area of Circle is {}",cir_area);

}