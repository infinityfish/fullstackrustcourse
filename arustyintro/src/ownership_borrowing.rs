

pub fn run() {
    let name = String::from("zoe");
    calculate_length(&name); // borrowed here.
    println!("The name is {}", &name);
    say_hi(&name);
}

fn calculate_length(name: &String) {
    println!("The name is {} characters long", name.len());
}

fn say_hi(name: &String) {
    println!("Hi {}, how are you?", name);
}