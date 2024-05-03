pub fn compound_main() {

    // Fixed size. But can handle a variety of types.
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;

    println!("The value of the tuple is {x}, {y}, {}", tuple.2);

    // Fixed size. Must be same type. Allocates data on stack.
    let basic_array: [i32; 5] = [1, 2, 3, 4, 5];
    // Initiates 12 elements with the value 6
    let same_basic_array = [6; 12];


    println!("Basic Array {:?}", basic_array);
    println!("Same Value Basic Array {:?}", same_basic_array);
}