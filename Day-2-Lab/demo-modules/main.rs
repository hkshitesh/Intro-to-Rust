// pub mod shape {
//     pub fn draw(name:String){
//         println!("Drawing Shape {}",name);
//     }
// }

pub mod math {
    pub mod geo {
        pub mod shape {
            pub fn draw_sh(name:String){
                println!("Drawing Shape using Nested Modules {}",name); 
            }
            pub fn display_sh(name:String){
                println!("Displaying Shape using Nested Modules {}",name); 
            }
        }
    }
}
//full qualifed path
// fn main()
// {
//     shape::draw("Circle".to_string());
// }


// use shape::draw;

// fn main()
// {
//     draw("Circle".to_string());
// }

// fn main()
// {
//     math::geo::shape::draw_sh("Sqaure".to_string());
//     math::geo::shape::display_sh("Sqaure".to_string());
// }

use math::geo::shape;
fn main()
{
    shape::draw_sh("Sqaure".to_string());
    shape::display_sh("Sqaure".to_string());
}