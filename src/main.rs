fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // x is now mutable.
    x = 6;
    println!("The new value of x is {}", x);

    const THREE_HOURS_IN_SECONDS:u32 = 60*60*3;

    println!("How many seconds in 3 hours? {THREE_HOURS_IN_SECONDS}")
}
