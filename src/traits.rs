
// creating a derive macro for all creation of basic structs
macro_rules! create_struct_with_fields {
    ($name:ident,$($field_name:ident : $field_type:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $( $field_name:$field_type,)*
        }
        impl $name {
            pub fn new($($field_name:$field_type),*)->Self {
                $name {
                    $( $field_name,)*
                }
            }
        }
    }
}
// creating a derivative macro for creation of struct with vectors and generic types
macro_rules! create_struct_with_vecs {
    ($name:ident, $t:ident) => {
        #[derive(Debug)]
        pub struct $name<$t> {
            pub vals: Vec<$t>,
        }
        impl<$t> $name<$t> {
            pub fn new(vals:Vec<$t>)->Self {
                $name {vals}
            }
        }
    }
}
//1
pub trait Describe {
    fn describe(&self) -> String;
}
//2
pub trait Area {
    fn area(&self) -> f64;
}
//3
pub trait Sum<T> {
    fn sum(&self) -> T;
}
//4 & 6
pub trait Shape {
    type Dimensions;
    type Numeric : std::fmt::Display;

    fn shape(&self) -> Self::Dimensions;
    fn area1(&self) -> Self::Numeric;
    fn perimeter(&self) -> Self::Numeric;
}
//5
pub trait Volume<T> {
    fn volume(&self)->T;
}
pub fn print_volume<T:Volume<U>+Describe,U:std::fmt::Display>(shape: T) {
    println!("The volume of the shape {:?} is: {}",shape.describe(),shape.volume());
}
//7
pub trait Shape3D<T>
    where 
    T:Describe+Area {}
//8
pub trait DefaultArea {
    fn area(&self)->i32 {
        println!("The default object has no area");
        0
    }
}
//9
pub trait Reportable {
    fn summary(&self)->String;
    fn details(&self)->String;
}
pub fn generate_report(reportable: &dyn Reportable) {
    println!("Summary:\n{}", reportable.summary());
    println!("Details:\n{}", reportable.details());
}
// for cleaner code from ChatGPT (the mod of structs part)
mod structs {
    use std::fmt::Debug;

    use super::*;
    //creating a generic describe trait for all debug structs (which is ofcourse created by the macro) also kinda solved problem 3
    impl<T:Debug> Describe for T {
        fn describe(&self)->String{
            format!("{:?}",self)
        }
    }
//1
    create_struct_with_fields!(Simple,name:String);
//2
    create_struct_with_fields!(Rectangle,length:f64,breadth:f64);
    impl Area for Rectangle {
        fn area(&self)->f64 {
            self.length*self.breadth
        }
    }
    create_struct_with_fields!(Circle,radius:f64);
    impl Area for Circle {
        fn area(&self)->f64 {
            self.radius.powi(2)*std::f64::consts::PI
        }
    }
//3
    create_struct_with_vecs!(Container,T);
    // create_struct_with_vecs!(Floats,f64);

    impl<U> Sum<U> for Container<U>
    where U:Copy+std::iter::Sum<U>
    {
        fn sum(&self)-> U{
            self.vals.iter().map(|c| c.clone()).sum()
        }
    }
//4 & 6
    impl Shape for Rectangle{
        type Dimensions=(f64,f64);
        type Numeric = f64;

        fn shape(&self)->Self::Dimensions {
            (self.breadth,self.length)
        }
        fn area1(&self)->Self::Numeric {
            self.area()
        }
        fn perimeter(&self) -> Self::Numeric {
            2.0*(self.breadth+self.length)
        }
    }

    impl Shape for Circle{
        type Dimensions=(f64);
        type Numeric = f64;
        fn shape(&self)->Self::Dimensions {
            (self.radius)
        }
        fn area1(&self)->Self::Numeric {
            self.area()
        }
        fn perimeter(&self) -> Self::Numeric {
            2.0*std::f64::consts::PI*self.radius
        }
    }
//5
    //remember can't implemetn shape for Cube
    create_struct_with_fields!(Cube,x:f64,y:f64,z:f64);
    impl Volume<f64> for Cube{
        fn volume(&self)->f64 {
            self.x*self.y*self.z
        }   
    }
//7 
    create_struct_with_fields!(Dodecahedron,edge_length:f64);
    impl Area for Dodecahedron {
        fn area(&self)->f64 {
            //formula for dodecahedron
            3.0 * (25.0 + 10.0 * (5.0 as f64).sqrt()).sqrt() * self.edge_length.powi(2)
        }
    }
    impl Shape3D<Dodecahedron> for Dodecahedron{}
//8
    create_struct_with_fields!(Square,length:f64);
    impl DefaultArea for Square {}
//9
    create_struct_with_fields!(User,name:String,email:String);
    impl Reportable for User {
        fn summary(&self) ->String {
            format!("User: {}",self.name)
        }
        fn details(&self)->String {
            format!("User: {}\nEmail: {}",self.name,self.email)
        }
    }
    create_struct_with_fields!(Product,name:String,price:f64,category:String);
    impl Reportable for Product {
        fn summary(&self) -> String {
            format!("Product: {}",self.name)
        }
        fn details(&self) -> String {
            format!("Product: {}\nPrice: {}\nCategory: {}",self.name,self.price,self.category)
        }
    }
    
}
fn main() {
    //1
    let a = structs::Simple::new("simple test!".to_string());
    println!("The summary of this struct is: {}",a.describe());
    println!();
    //2
    let rect1 = structs::Rectangle::new(4.0,5.0);
    let circ1 = structs::Circle::new(5.0);

    println!("The area for rectangle {:?} is {}",rect1.describe(),rect1.area());
    println!("The area for circle {:?} is {:.5}",circ1.describe(),circ1.area());
    println!();
    //3
    let arr_ints = structs::Container::new(vec![1,2,87,43,12]);
    let arr_floats = structs::Container::new(vec![3.0,6.7,9.0,23.1]);

    println!("The sum of array of integers {:?} is {}",arr_ints.describe(),arr_ints.sum());
    println!("The sum of array of floating point numbers {:?} is {}",arr_floats.describe(),arr_floats.sum());
    println!();
    //4
    println!("The shape of the rectangle rect1 is {:?}",rect1.shape());
    println!("The shape of the circle circ1 is {}",circ1.shape());
    println!();
    //5
    let cube1 = structs::Cube::new(4.0,5.0,7.0);
    print_volume(cube1);
    println!();
    //6
    println!("The perimeter for rectange {:?} is {}",rect1.describe(),rect1.perimeter());
    println!("The perimeter for circle {:?} is {:.5}",circ1.describe(),circ1.perimeter());
    println!();
    //7
    let dodec = structs::Dodecahedron::new(4.5);
    println!("The area for dodecahedron is {:?} is {:.2}",dodec.describe(),dodec.area());
    println!();
    //8 
    let square1 = structs::Square::new(4.3);
    println!("The default value of overwirrden shape Square: {:?} is {}",square1.describe(),square1.area());
    //9
    let user = structs::User::new("johndoe".to_string(),"johndoe@example.com".to_string());
    let product = structs::Product::new("Laptop".to_string(), 999.99,"Electronics".to_string());

    generate_report(&user);
    generate_report(&product);


}
