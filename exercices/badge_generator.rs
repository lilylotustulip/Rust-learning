fn main () {
    let username = String::from("cookie");
    let clearance_level:i32 = 15;
    let badge = generate_badge(username, clearance_level);
    println!("the badge is,  {badge} ");
    
}


fn generate_badge(username:String, clearance_level:i32) ->String{
    format!("Username: {}, level: {}", username, clearance_level)
}