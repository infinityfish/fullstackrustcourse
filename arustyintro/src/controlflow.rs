

pub fn run() {
    let age = 15;
    if age > 18 {
        println!("You can drink");
    } else if age == 18 {
        println!("You are barely 18");
    } else {
        println!("You cannot drink");
    }

    let myage = 21;
    match myage {
        18 => println!("Hey you are 18"),
        21 => println!("Hey you are 21 now and you can drink"),
        _  => println!("We dont know your age")
     }
}