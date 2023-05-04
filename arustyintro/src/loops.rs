  
pub fn run(){
    let mut x = 10;
    while x > 2 {
    println!("Number is {}", x);
    x = x - 2;
    println!("I am done");
  }

  let scores = [6,8,10];
  for score in scores.iter() {
    println!("The score is {}", score);
  }

  for num in 2..5 {
    println!("Number - {} x 2 = {}", num, num*2 );
  }

}