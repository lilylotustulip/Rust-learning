// declare the main function
fn main(){
// within the main function initialize 3 variables.
// name, drink and bill.
    let name = String::from("customer A");
    let drink = String::from("latte");
    let mut bill:i32 = 0;
// call the bill_calculator with its parameters.
    bill_calculator(&mut bill);
    receipt(name, &mut bill, &drink);
}


// declare a bill_calculator function
fn bill_calculator(bill:&mut i32){
// add bill for the drink 
    *bill += 5;
    println!("the bill is now: {}", bill);
}

//declare a receipt function
//pass all the variables as parameters.
fn receipt(name:String, bill:&mut i32, drink:&String) {
// print the final receipt.
    println!("name: {} | drink: {} | bill: {}", name, drink, bill);
}


