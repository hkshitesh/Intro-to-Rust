// struct Employee{
//     name:String,
//     company:String,
//     age:u32
// }

// fn main()
// {
//     let mut emp1 = Employee{
//         name:String::from("John"),
//         company:String::from("comp1"),
//         age:34
//     };
//     let mut emp2 = Employee{
//         name:String::from("Kerry"),
//         company:String::from("comp2"),
//         age:45
//     };    
    
//     // display(emp1);
//     // display(emp2);
//     let old_emp = older_emp(emp1,emp2);
//     println!("Older Employee is");
//     display(old_emp);
    
// }

// fn display(emp:Employee)
// {
//     println!("Name is {} Company is {} and Age is {}",emp.name, emp.company,emp.age);
// }

// fn older_emp(e1:Employee, e2:Employee)->Employee{
//     if e1.age>e2.age{
//         return e1;
//     }else{
//         return e2;
//     }
// }


struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self)->u32{
        return self.width*self.height;
    }
}

fn main()
{
    let rect = Rectangle{
        width:10,
        height:20
    };
    println!("width is {} height is {} and area is {}",rect.width, rect.height, rect.area());
}