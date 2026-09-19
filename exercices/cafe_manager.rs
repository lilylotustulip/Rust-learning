struct Customer{
    name:String,
    balance:i32,
}
fn main(){
    let mut customer1 = Customer{
        name:String::from("Alex"),
        balance:0,
    };

    add_item(&mut customer1);
    pay_tab(customer1);
}

fn add_item(c: &mut Customer){
    let item:i32 = 4;
    c.balance += item;
}

fn pay_tab(mut c: Customer){
    let payed_bill: i32 = 4;
    c.balance -= payed_bill;
    println!("tab closed for customer {}, payed bill {}", c.name, c.balance);
}