/* run all functions in lib.rs */

use calc::add;
use calc::div;
use calc::mul;
use calc::sub;

fn main() {
    println!("Hello, world!");
    println!("add(1,2)={}", add(1, 2));
    println!("sub(1,2)={}", sub(1, 2));
    println!("mul(1,2)={}", mul(1, 2));
    println!("div(1,2)={}", div(1f32, 2f32));
}
