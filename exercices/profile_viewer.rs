fn main() {
    let username = String::from("cookie");
    let score: i32 = 100;
    display_profile(&username, score);
    display_profile(&username, score);// called twice with no error, which proves main retained ownership of the variables
    println!("username : {}", username);
    
    
}

fn display_profile(username: &String, score: i32) {
    println!("Player: {} | Score: {}", username, score);
}