fn functions_chapter() {
    print_labeled_measurement(16, 'w');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}