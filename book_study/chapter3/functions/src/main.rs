fn main() {
    println!("running...");

    another_function(2);
    temperature(20, 'C');
    let x = number();
    println!("the number represents: {x}");
}

fn another_function(a: i32) {
    println!("the value of a is: {a}");
}

fn temperature (value: u32, temp_type: char) {
    println!("the temperature is: {value}{temp_type}");
}

fn number() -> i32 {
    2
}