
mod my_mod;

use crate::my_mod::fun;
use a_new_project::add;

fn main() {
    println!("Hello, world!");
    println!("add(1,2)={}", add(1, 2));

    fun();

}
