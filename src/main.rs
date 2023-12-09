use dotenv::dotenv;
use std::env;
fn main() {
    // load environment variables from .env file
    dotenv().ok();
    let data: String = env::var("DATA").expect("cannot find data in env");
    println!("{}", data);
}
