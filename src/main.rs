#![allow(dead_code, unused_mut, unused_variables)]
mod a_fundamentals {
    pub mod variables;
    pub mod scopes;
    pub mod memory_safety;
    pub mod exercise_variables;
    pub mod functions;
    pub mod exercise_functions;
}

mod b_primitive_types {
    pub mod exercise_simple_types;
    pub mod exercise_controlflow_string;
}

mod c_heart_of_rust {
    pub mod ownership_references;
}

use a_fundamentals::*;
use b_primitive_types::*;
use c_heart_of_rust::*;
use rand::{Rng, thread_rng};

fn main() {
    // variables::variables();
    // scopes::scopes();
    // memory_safety::memory_safety();
    // exercise_variables::exercise_variables();
    // println!("{}", functions::do_stuff(2, 6))

    // let width = 4;
    // let height = 7;
    // let depth = 10;
    //
    // let area = exercise_functions::area_of(width, height);
    // let volume = exercise_functions::volume(width, height, depth);
    //
    // println!("Area is {} and volume is {}", area, volume);
    // let x = thread_rng().gen_range(0..100);
    // println!("{}", x);

    // let coords: (f32, f32) = (6.3,  15.0);
    // let coords_array: [f32; 2] = [coords.0, coords.1];
    // let series = [13,2,3,4,5,6,7,8,9,10,10];
    // let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // print_difference(coords.0, coords.1);
    // print_array(coords_array);
    // ding(series[1])
    // on_off(mess.2[1].0);

    // let args: Vec<String> = std::env::args().skip(1).collect();
    // for arg in args {
    //     if arg == "sum" {
    //         exercise_controlflow_string::sum();
    //     } else if arg == "double" {
    //         exercise_controlflow_string::double();
    //     } else {
    //         exercise_controlflow_string::count(arg);
    //     }
    // }

    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    ownership_references::ownership_references(arg);
}