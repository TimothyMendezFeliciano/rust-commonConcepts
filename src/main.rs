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
}
