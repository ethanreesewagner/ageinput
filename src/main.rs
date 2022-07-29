use input_macro::input;
use chrono::prelude::*;
use std::env;

fn main(){
    env::set_var("RUST_BACKTRACE", "full");
    let dt:i128=Utc::now().year().into();
    let name = input!("What's your name? ");
    let age:i128 = input!("What's your age? ").parse().unwrap();
    if age<100{
        println!("{} will be 100 
        years old by the year {}.",name,dt+100-age);
    }else{
        println!("{} became 100 years
         old on the year {}.",name,dt+100-age);
    }
}