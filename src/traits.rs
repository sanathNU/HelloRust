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
//4
pub trait Shape {
    type Dimensions;

    fn shape(&self) -> Self::Dimensions;
}
//5
pub trait Volume<T> {
    fn volume(&self)->T;
}
pub fn print_volume<T:Volume<U>+Describe,U:std::fmt::Display>(shape: T) {
    println!("The volume of the shape {:?} is: {}",shape.describe(),shape.volume());
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
//4 
    impl Shape for Rectangle{
        type Dimensions=(f64,f64);

        fn shape(&self)->Self::Dimensions {
            (self.breadth,self.length)
        }
    }
    impl Shape for Circle{
        type Dimensions=(f64);
        fn shape(&self)->Self::Dimensions {
            (self.radius)
        }
    }
//5
    create_struct_with_fields!(Cube,x:f64,y:f64,z:f64);
    impl Volume<f64> for Cube{
        fn volume(&self)->f64 {
            self.x*self.y*self.z
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
    println!("The area for rectangle {:?} is {:.5}",circ1.describe(),circ1.area());
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

}
