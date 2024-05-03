pub fn control_flows(n: u32) {
    let value: bool = if n % 2 == 0 { true } else { false };

    if value {
        println!("{n} is an even number")
    } else {
        println!("{n} is an odd number")
    }
}