fn safe_divide(num: f64, deno: f64) -> Option<f64> {
    if deno == 0.0 {
        None
    }
    else{
        Some(num / deno)
    }
}

fn print_div_result(result: Option<f64>){
    match result {
        None => println!("cannot divide by zero"),
        Some(val) => println!("result: {val}"),
    }
}

fn main(){
    let res1 = safe_divide(4.0, 2.0);
    print_div_result(res1);

    let res2 = safe_divide(4.0, 0.0);
    print_div_result(res2);
}