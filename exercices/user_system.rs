struct user {
    full_name : String,
    status_tier : String,
    login_count: i32,
}
fn main(){
    let mut user1 = user {
        full_name: String::from("cookie cookies"),
        status_tier: String::from("free"),
        login_count: 0,
    };
    
}
fn upgrade(&mut status_tier: String) -> user{
    user {
        status_tier: String::from("VIP"),
        ..
    }
}

fn username_display(&full_name: String){
    println!("username: {}", fullname.0 );
}

fn deletion(){

}