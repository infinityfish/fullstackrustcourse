

pub struct Car {
    make: String,
    year: u32
}

pub struct LatLong (f64, f64);

pub fn run(){
    let car1 = Car {
        make: String::from("GMC Acadia"),
        year: 2021
    };
    println!("My car is a {} {}", car1.year, car1.make);

    let location = LatLong(84.556, 33.66);
    println!("Location is {:?}, {:?}", location.0, location.1);
}