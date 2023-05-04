

pub fn run() {
    let fruits = vec!["apples", "mangoes", "oranges"];
    let tuple = ("eric", 25, true);
    println!("First fruit from Vec is: {}", fruits[0]);
    println!("{} is {} years old and smart: {}", tuple.0, tuple.1, tuple.2);
}