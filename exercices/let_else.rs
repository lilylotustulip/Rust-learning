enum UserRole {
     Admin(String), 
     Guest, 
}

// Refactor this function to use \`let...else\` instead of nested \`if let\`
fn get_admin_name(role: UserRole) -> String { 
    let UserRole::Admin(name) = role  else {
       return String::from("Access Denied"); 
    };
    name
}
fn main(){
    let admin_user = UserRole::Admin(String::from("cookie"));
    println!("{}", get_admin_name(admin_user));

    let guest_user = UserRole::Guest;
    println!("{}", get_admin_name(guest_user));
}