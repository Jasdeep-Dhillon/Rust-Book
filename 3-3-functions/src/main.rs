fn main() {
    println!("Hello, World");
    another_function();
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn another_function() {
    println!("Printing from another funciton");
}