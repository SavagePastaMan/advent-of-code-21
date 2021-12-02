mod solutions;
pub(crate) mod helper;

use solutions::day01::*;

fn main() {
    let data = input();
    println!("{:?}", one(data));
}
