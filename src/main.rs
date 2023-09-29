#![allow(unused_variables)]
mod a_fundamentals {
    pub mod variables;
    pub mod scopes;
    pub mod memory_safety;
    pub mod exercise_variables;
    pub mod functions;
    pub mod exercise_functions;
}

use a_fundamentals::*;
use rand::{Rng, thread_rng};

fn main() {
    // variables::variables();
    // scopes::scopes();
    // memory_safety::memory_safety();
    // exercise_variables::exercise_variables();
    // println!("{}", functions::do_stuff(2, 6))

    let width = 4;
    let height = 7;
    let depth = 10;

    let area = exercise_functions::area_of(width, height);
    let volume = exercise_functions::volume(width, height, depth);

    println!("Area is {} and volume is {}", area, volume);
    let x = thread_rng().gen_range(0..100);
    println!("{}", x);
}