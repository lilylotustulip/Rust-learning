fn main() {
    let mut health:i32 = 50;
    let mut score = String::from("cookie");
    let bonus:i32 = 10;
    println!("score: {}, health: {}", score, health);
    apply_power_up(&mut health, &mut score, bonus);
    println!("score: {}, health: {}", score, health);
}

fn apply_power_up(health:&mut i32, score:&mut String, bonus:i32 ) {
    *health += bonus;
    *score = String::from("cookie");

}