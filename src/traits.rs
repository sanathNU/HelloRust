// creating a derive macro for all creation of structs
macro_rules! create_struct {
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
//1
pub trait Describe {
    fn describe(&self) -> String;
}
//2
pub trait Area {
    fn area(&self) -> f64;
}
// for cleaner code from ChatGPT
mod structs {
    use super::*;
//1
    create_struct!(Simple,name:String);
    impl Describe for Simple{
        fn describe(&self)->String{
            format!("{:?}",&self)
        }
    }
//2
    create_struct!(Rectangle,length:f64,breadth:f64);
    impl Area for Rectangle {
        fn area(&self)->f64 {
            self.length*self.breadth
        }
    }
    impl Describe for Rectangle{
        fn describe(&self)->String{
            format!("{:?}",&self)
        }
    }
    
    create_struct!(Circle,radius:f64);
    impl Area for Circle {
        fn area(&self)->f64 {
            self.radius.powi(2)*std::f64::consts::PI
        }
    }
    impl Describe for Circle{
        fn describe(&self)->String{
            format!("{:?}",&self)
        }
    }
}
fn main() {
    //1
    let a = structs::Simple::new("simple test!".to_string());
    println!("The summary of this struct is: {}",a.describe());
    //2
    let rect1 = structs::Rectangle::new(4.0,5.0);
    let circ1 = structs::Circle::new(5.0);

    println!("The area for rectangle {:?} is {}",rect1.describe(),rect1.area());
    println!("The area for rectangle {:?} is {}",circ1.describe(),circ1.area());


}
