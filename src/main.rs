#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut serv = Nickel::new();

    serv.get("/", middleware!("This is the / handler"));

    serv.listen("127.0.0.1:8000");
}
