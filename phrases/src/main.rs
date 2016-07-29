extern crate phrases;

use phrases::english::{greetings, farewells};
use phrases::chinese;

fn main(){
    println!("Hello in english {}", greetings::hello());
    println!("Goobye in english {}", farewells::goodbye());

    println!("Hello in chinese {}", chinese::hello());
    println!("Goobye in chinese {}", chinese::goodbye());
}
