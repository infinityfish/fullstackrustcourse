
pub fn run() {
    let lang = "Rust";
    // let mut x: i32 = 5;
    let x = 5;
    // let float: f64 = 2.84;
    // let inter: i64 = float/4.0;// not working
    // x = x + 3;
    let y = x + 2;
    let z = x*2;
    let candrink = true;
    let fname = String::from("Eric");
    println!("My lang is {}", lang);
    println!("Y is {}", y);
    println!("X is {}", x);
    println!("Z is {}", z);
    println!("Can drink is {}", candrink);
    println!("My other language is {}", lang);
    say_hello(fname);
    // say_hi(fname);
}

fn say_hello(name: String) -> String {
    println!("Hello My name is {}", name);
    name
}

fn say_hi(name: String) -> String {
    println!("hi I am is {}", name);
    name
}
