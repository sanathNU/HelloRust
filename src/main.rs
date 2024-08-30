#![allow(warnings)]

mod level_1;
mod level_2;
mod level_3;
mod level_4;
mod level_5;
mod level_6;
mod level_7;
mod level_8;
mod level_9;
mod level_10;
// use util::GeoObject;

//default function to prevent any testing
// fn main() {
//     println!("Namaste World!");
// }

#[tokio::main]
async fn main() {
    // if let Err(e) = level_10::web_scraping_test().await {
    //     eprintln!("Error during web scarping: {}",e);
    //     std::process::exit(1);
    // }
    level_10::GeoObject!(Location,x:f64,y:f64);
    let location1 = Location::new(45.6,34.5);
    location1.read();
}