use core::fmt;
use std::borrow::BorrowMut;
use std::sync::{Arc,Mutex};
use std::thread;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::io;
use std::collections::HashMap;
use reqwest;
use scraper::{Html, Selector};
use tokio::task;

// function to use threads for calculating factorial of a number
fn factorial(n:u32) -> u32 {
    (1..=n).product()
}
// 🟩public wrapper function
pub fn multiple_factorial() {
    // let numbers = vec![5,6,7,8,9];
    println!("Enter the numbers whose factorial should be calculated in a whitespace seperated string");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers:Vec<u32>= input.trim().split_whitespace().map(|c| c.parse::<u32>().expect("expected a number")).collect();
    let results = Arc::new(Mutex::new(vec![0; numbers.len()]));

    let mut handles = vec![];

    for (i,&num) in numbers.iter().enumerate() {
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let result = factorial(num);
            let mut results = results.lock().unwrap();
            results[i] = result;
        });

        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    for (i,&num) in numbers.iter().enumerate() {
        println!("Factorial of {} if {}",num, results[i]);
    }

}

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(String),
}
impl fmt::Display for AppError {
    fn fmt(&self, f:&mut fmt::Formatter)-> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f,"IO error: {}",err),
            AppError::ParseError(ref msg) => write!(f,"Parse error: {}",msg),
        }
    }
}
impl Error for AppError{}

impl From<io::Error> for AppError {
    fn from(err:io::Error) -> Self {
        AppError::IoError(err)
    }
}
// 🟩public wrapper function
pub fn reading_integers(path:&str)->Result<Vec<i32>, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut integers = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for token in line.split_whitespace(){
            match token.parse::<i32>() {
                Ok(num) => integers.push(num),
                Err(_) => return Err(AppError::ParseError(format!("Invalid Integer: {}",token))),
                }
            }
        ;}
    Ok(integers)
}
// creating a task manager for lifetime problem
struct Task {
    name: String,
    description: String,
    priority: u8,
}
impl Task {
    fn new(name: &str, description: &str, priority: u8) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            priority: priority
        }

    }

    fn display(&self) {
        println!("Name: {}\nDescription: {}\nPriority: {}\n",self.name, self.description, self.priority);
    }
}

struct TaskManager<'a> {
    tasks: HashMap<String,Task>,
    //what is this?
    _phantom: std::marker::PhantomData<&'a ()>
}
impl<'a> TaskManager<'a> {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            _phantom: std::marker::PhantomData,
        }
    }
    fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.name.clone(),task);
    }
    fn get_task_details(&self, name: &str) -> Option<&Task> {
        self.tasks.get(name)
    }
    fn display_tasks(&self){
        if self.tasks.is_empty() {
            println!("No Tasks Available");
        }
        for task in self.tasks.values() {
            task.display();
        }
    }
}
// 🟩public wrapper function
pub fn task_manager_checker() {
    println!("This function checks the taskmanaging function");
    let mut manager = TaskManager::new();

    let task1 = Task::new("Wake up Early","Wake up with 7 hours of sleep before 7AM",9);
    let task2 = Task::new("Breakfast","Eat healthy breakfast 90 mins after waking up",6);

    manager.add_task(task1);
    manager.add_task(task2);

    manager.display_tasks();
}

//creating a web_scraping bot to capture trending wikipedia pages, and report single line summaries of it.
// pub fn web_scraper(url:&str) -> 

// pub fn scrapper() -> Result<(), reqwest::Error> {
//     let url = "https://wikipedia.org";
//     let client = reqwest::blocking::Client::new();

//     let response = client.get(url).send()?;
//     println!("Status request: {}",response.status());

//     let body = response.text()?;
//     println!("Response body:\n{}",body);

//     Ok(())
// }

async fn fetch_url(url: &str) -> Result<String,reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
fn extract_title(html_content: &str) -> Option<String>  {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("title").ok()?;
    let title_element = document.select(&selector).next()?;
    Some(title_element.text().collect::<Vec<_>>().concat())
}
pub async fn web_scraping_test() -> Result<(), reqwest::Error> {

    let urls = vec![
        "https://www.wikipedia.org",
        "https://www.rust-lang.org",
        "https://www.github.com",
    ];
    let mut tasks = vec![];

    for url in urls   {
        let url = url.to_string();
        let task = task::spawn(async move {
            match fetch_url(&url).await {
                Ok(content) => {
                    if let Some(title) = extract_title(&content) {
                        println!("Title from {}: {}",url, title);
                    } else {
                        eprintln!("Failed to extract title from {}",url);
                    }
                }
                Err(e) => eprintln!("Failed to fetch {}: {}",url,e),
            }
        });
        tasks.push(task);
      };
    for task in tasks {
        task.await.expect("Task failed");
    }

    Ok(())
}

// **CRUD operations macro rust**: creating a geolocation mainly for testing?

// #[macro_export]
macro_rules! GeoObject {
    ($name:ident,$($field_name:ident : $field_type:ty),*) => {
        struct $name {
            $( $field_name: $field_type,)*
        }
        impl $name {
            fn new($($field_name:$field_type),*)->Self {
                $name {
                    $( $field_name,)*
                }
            }
            fn read(&self){
                $(
                    println!("{}: {:?}",stringify!($field_name), &self.$field_name);
                )*
            }
            fn update(&mut self,$($field_name:$field_type),*){
                $(
                    self.$field_name = $field_name ;
                )*
            }
        }
        // impl Drop for $name {
        //     fn drop(&self){
        //         println!("Destroying struct");
        //     }
        // }
        
    };
}

pub(crate) use GeoObject;

// trait library for calculating areas of different objects
macro_rules! create_shapes {
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
pub trait Describe {
    fn describe(&self) -> String;
}
trait Area {
    type Numeric: std::fmt::Display;

    fn area(&self)->Self::Numeric;
}
impl<T:std::fmt::Debug> Describe for T {
    fn describe(&self)->String{
        format!("{:?}",self)
    }
}

create_shapes!(Circle,radius:f64);
impl Area for Circle {
    type Numeric = f64;
    fn area(&self)->Self::Numeric{
        std::f64::consts::PI*self.radius.powi(2)
    }
}

create_shapes!(Triangle,a:f64,b:f64,c:f64);
impl Area for Triangle {
    type Numeric = f64;
    fn area(&self)->Self::Numeric{
        let s = (self.a+self.b+self.c)/2.0;
        (s*(s-self.a)*(s-self.b)*(s-self.c)).sqrt()
    }
}
create_shapes!(Rectangle,length:f64,breadth:f64);
impl Area for Rectangle {
    type Numeric = f64;
    fn area(&self)->Self::Numeric{
        self.length*self.breadth
    }
}


// 🟩public wrapper function
pub fn macro_test() {
    // Create instances of the shapes
    let circ1 = Circle::new(5.0);
    let tri1 = Triangle::new(3.0, 4.0, 5.0);
    let rect1 = Rectangle::new(4.0, 5.0);

    // Print descriptions and areas
    println!("Circle Description: {}", circ1.describe());
    println!("Circle Area: {}", circ1.area());

    println!("Triangle Description: {}", tri1.describe());
    println!("Triangle Area: {}", tri1.area());

    println!("Rectangle Description: {}", rect1.describe());
    println!("Rectangle Area: {}", rect1.area());
}

