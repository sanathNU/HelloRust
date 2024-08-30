// Basic macro definition
macro_rules! BasicMacro {
    ($expression:expr) => {
         println!("This is the string,{}",$expression);
        };
}

// Macro with Conditional Patterns
macro_rules! ifmacro {
    ($x:expr) => {
        println!("This is the base single case, the value is this: {}",$x)
    };
    ($x:expr,$y:expr) => {
        println!("Secondary case, more exprs needed. The first is {}",$x);
        println!("the second string is {}",$y);
    }
}
// Macro with Repetition
macro_rules! repmacro {
    //base case
    ($x:expr) => {println!("expr is :{}",$x)};
    ($x:expr,$($y:expr),+) => {
        repmacro!($x);
        repmacro!($($y),+);
    }
}
// Macro with Variable Arguments
macro_rules! fnmacro {
    // first is function name
    ($x:ident,$($y:expr),+) => {
        fn $x() {
            let a = vec![$($y),+];
            for item in a {
                println!("{}",item);
            }
        }
    }
}
// Simple Struct & Method Generation
macro_rules! structmacro1 {
    ($x:ident) => {
        struct $x {
            name: String,
        }
        impl $x {
            fn new(name:&str)-> Self {
                $x {
                    name:name.to_string(),
                }
            }
            fn get_name(&self)->&str{
                &self.name
            }
        }
    }
}
// Struct Field Initilzation
macro_rules! struct2 {
    ($name:ident,$($field_name:ident : $field_type:tt),+) => {
        struct $name {
            $(
                $field_name: $field_type,
            )*
        }
        impl $name {

            fn new($($field_name:$field_type),*)->Self{
                $name {
                    $(
                        $field_name,
                    )*
                }
            }
            fn print_val(&self) {
                // println!("Fruit name: {}\n Fruit price: {}",self.name,self.price);
                $(
                    println!("{}: {:?}",stringify!($field_name),&self.$field_name);
                )*
            }
        }
    }
}
// Macro with Syntax Checks
macro_rules! struct3 {
    ($name:ident,$item: ty)=> {
        #[derive(Debug)]
        struct $name($item);


        impl $name {
            fn new(value:$item)->Self{
                $name(value)
            }

            fn get_value(&self)->&$item {
                &self.0
            }
        }
    }
}
// Implemnting traits via macros
macro_rules! structTrait {
    ($name:ident,$($field_name:ident : $field_type:ty = $default_val:expr),+) => {
        #[derive(Debug)]
        struct $name {
            $( $field_name:$field_type, )*
        }
        impl Default for $name{
            fn default() -> Self {
                Self {
                    $(
                        $field_name: $default_val,
                    )*
                }
            }
        }
    }
}

// function to test macros
// this is just a remeniscent of the file I used to debug this code
// fn main() {
//     //1
//         ifmacro!("1","2");
//         ifmacro!("1only");
//     //2
//         repmacro!("1","2","3");
//     //3
//         var();
//         structmacro1!(fruit);
//         let k = fruit::new("mango");
//         println!("The name of the fruit is {}",k.get_name());
//     //4
//         struct2!(Fruit,name:String,price:u32);
    
//         let items = vec![
//             ("apple".to_string(),100),
//             ("orange".to_string(),90),
//         ];
    
//         let fruits:Vec<Fruit> = items.iter().map(|c| Fruit::new(c.0.clone(),c.1)).collect();
    
//         for fruit in fruits {
//             fruit.print_val();
//         }
//     //5
//         struct3!(debuggableInt,u32);
//         struct3!(debuggableStr,String);
//         //from https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
//         let instance1 = debuggableStr::new("string".to_string());
//         let instance2 = debuggableInt::new(3);
//         println!("Trying to print macro generated struct instance 1: {:#?}",instance1);
//         println!("Trying to print macro generated struct instance 2: {:?}",instance2);
//     //6
//         structTrait!(
//             Config,
//             host: String = "localhost".to_string(),
//             port: u16 = 8000
//             );
//         let default_config = Config::default();
    
//         println!("{:?}",default_config);
    
//     }