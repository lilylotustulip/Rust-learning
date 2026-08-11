fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!"); // error: you should do s2 because s1 has been moved to s2

}