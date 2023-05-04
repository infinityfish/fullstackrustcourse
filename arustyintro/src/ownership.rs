

pub fn run(){
    //The 3 Ownership rules

    let fname = String::from("olu");
    // 1. fname is the owner of "olu"
     // 2. There can be only one owner at a time
    say_hi(fname);
     // 3. When the owner goes out of scope, the value will be dropped
    //  println!("Hi {}, how are you?", fname);  //fname is out of scope already,
}

fn say_hi(name: String) {
    println!("Hi {}, how are you?", name);
}



