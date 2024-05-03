pub fn functions_chapter() {
    print_labeled_measurement(16, 'w');

    let result = return_value_example_times_five(2);

    println!("Return Value multiplied by 5: {result}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
    // Statements are instructions that perform some action and do not return a value.
    let y = 6;

    // The following produces error. Since assigning a value is a statement.
    // let x = (let z = 12);

//     Expressions evaluate to a resultant value.
//     5 + 6 is an expression. Since it evluates to 11.
    let x = {
        let z = 3;
        // DON'T add semicolon (;). Adding ; turns it into a statement, and does not return the value.
        z + 1
    };
}

// We don't add semicolon at the end. Most functions return the last expression implicitly.
fn return_value_example_times_five(n: i32) -> i32 {
    n * 5
}