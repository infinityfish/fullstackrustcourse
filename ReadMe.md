Source code for fullstackrust course.

#Setting up VSCode
crates - see versions
rust-analyzer -
Error Lens - inlays errors

#Rust intro course
Why Rust? Consistently the most loved programming language as per Stackoverflow for the past several yrs
Fast, Secure, General Programming language: systems programming, databases, OS programming, iot, web api, frontend, tooling,
Compiled very fast
Strongly type - secure, less bugs. Bugs discovered at compile time.
Great ecosystem
Crates.io just like NPM for JS
Cargo package system.

- variables: immutable, mutable
- scalar data types:
  bool, integers, floats (nums with decimal) - all mathematic operations

- Compound data types:
  let array = [1,2,3];
  array[0];
  let tuple = (1, "man", true);
  tuple.0;
  an empty tuple is called a unit

- functions
- struct
  struct Car {
  name: String,
  age: u32
  }

- traits are behavior descriptions

let car1 = Car {
name: String::from("Audi a3"),
age: 6
};
car1.age;
tuple struct:
struct LatLong (u32, u32);
let location = LatLong(84,30);
println!("Location is {:?}, {:?}", location.0, location.1);

- Enum: list variants of data
  The Option enum is built in with standard lib
  enum Option<T> {
  Some<T>,
  None
  }

- Control Flow: if, else, match
  let age = 18;
  if age > 18 {
  println!("You can drink");
  } else if age = 18 {
  println!("You are barely 18");
  } else {
  println!("You cannot drink");
  }

- Control Flow: match - is similar to switch in other languages
  let age = 18;

match age {
18 => println!("Hey you are 18"),
21 => println!("Hey you are 21 now and you can drink"),
\_ => println!("We dont know your age")
}

- Loops
  loop { println!("printing for ever);
  break;}

let mut x = 10;
while x != 0 {
println!("Number is {}", x);
x -= 1;
println!("I am done");
}

let scores = [2,4,6,8,10];
for score in scores.iter() {
println!("The score is {}", score);
}

for num in 4..7 {
println!("Number x 2 is {}", num\*2 );
}

- Error handling
  using the panic macro
  panic!("There was an error");

use the Option enum
use Result enum
enum Result<T, E> {
OK(T),
Err(E)
}

The ? Operator

- Ownership
  Each value has one Owner: let age =20; age owns 20.
  There can only be one owner at a time
  If owner goes out of scope, value is dropped

- Borrowing

#Tools needed

1. Your wasm target:
   rustup target add wasm32-unknown-unknown
2. Trunk: WASM web application bundler for Rust
   cargo install trunk

3. cargo-watch:
   cargo install cargo-watch
   cargo watch -x run

mkdir axum-yew-app
cd axum-yew-app
git init
cargo new --bin server2 --vcs none
cargo new --bin frontend --vcs none

===================================
####Connecting frontend and server ####
create and add to Cargo.toml
[workspace]

members = [
"server", "frontend"
]

#create and add to .gitignore
target/
dist/
.env

create frontend/Trunk.toml and add contents
[build]
target = "index.html"
dist = "../dist"

now you can run: trunk serve

# now go to server/main.rs

axum::response::IntoResponse
axum::http::StatusCode,
use std::io;

//serving frontend static files
let serve_dir = ServeDir::new("../frontend/dist").not_found_service(ServeFile::new("../dist/frontend/index.html"));
let serve_dir = get_service(serve_dir).handle_error(handle_error);

# add to router

        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir.clone());

# add handle error fn

async fn handle_error(\_err: io::Error) -> impl IntoResponse {
(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

# change current / to /home

# cd into backend: cargo watch -x run

cd into server and Cargo add

# got to project dir and run

cargo run --bin server
#cd into server and run
cargo watch -x run

# in frontend dir

cargo install trunk

rustup target add wasm32-unknown-unknown
cargo add yew --features yew/csr
