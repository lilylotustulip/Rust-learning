use std::io;

fn main(){
let mut a_str = String::new();

println!("Enter the value of a: ");

io::stdin().read_line(&mut a_str).expect("Failed to read");
let a : u32 = a_str.trim().parse().expect("Invalid input");

let mut b_str = String::new();

println!("Enter the value of b: ");

io::stdin().read_line(&mut b_str).expect("Failed to read");
let b : u32 = b_str.trim().parse().expect("Invalid input");

let mut n_str = String::new();

println!("Enter the value of n: ");

io::stdin().read_line(&mut n_str).expect("Failed to read");
let n : u32 =n_str.trim().parse().expect("Invalid input");


let mut sigma_sum = 0;

for k in 0..n {
    let a_power = a.pow(n-1-k);
    let b_power = b.pow(k);
    let term = a_power * b_power;
    sigma_sum += term;

}

let result = (a - b) * sigma_sum;
println!("The result is: {}", result);
}