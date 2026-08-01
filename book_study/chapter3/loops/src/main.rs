fn main() {
    
    coutdown_loop();
    countdown_while();
    countdown_for();
}

fn coutdown_loop() {
    let mut number = 3;
    loop {
        println!("the number is {number}");
        number -= 1;

        if number == 0 {
            break
        }
    }
    println!("countdown of loop has run successfully!");
}

fn countdown_while() {
    let mut number = 3;
    while number > 0 {
        println!("the number is: {number}");
        number -= 1;
    }
    println!("countdown of the while loop run successfully!");
}

fn countdown_for() {
    let mut number = 3;
    for number in (1..=3).rev() {
        println!("the number is {number}");
    }
    println!("countdown of the for loop run successfully!");
}