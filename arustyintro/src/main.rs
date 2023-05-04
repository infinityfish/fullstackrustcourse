
mod variables;
mod compound;
mod functions;
mod structs;
mod controlflow;
mod loops;
mod ownership;
mod ownership_borrowing;

fn main() {
    ownership_borrowing::run();
    println!("Hello from main");
}
