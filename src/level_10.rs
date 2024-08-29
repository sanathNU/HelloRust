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
// 🟩public wrapper function
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
