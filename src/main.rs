use rand::{random, Rng};
use crate::compund_types::compound_main;
use crate::control_flow::control_flows;
use crate::functions_chapter::functions_chapter;
use crate::scalar_types::scalar_main;

mod scalar_types;
mod compund_types;
mod functions_chapter;
mod control_flow;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // x is now mutable.
    x = 6;
    println!("The new value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("How many seconds in 3 hours? {THREE_HOURS_IN_SECONDS}");

    let shadow_x = 18;

    let shadow_x = shadow_x + 9;
    {
        let shadow_x = shadow_x * 2;
        println!("The value of shadow_x in the inner scope is {shadow_x}");
    }

    println!("The value of shadow_x is {shadow_x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces in a shadow = {spaces}");

    // Causes compile time error because of incorrect type
    // let mut spaces = "       ";
    // spaces = spaces.len();

    scalar_main();

    compound_main();

    functions_chapter();

    let random = rand::thread_rng().gen_range(1..=100);
    control_flows(random);
}
