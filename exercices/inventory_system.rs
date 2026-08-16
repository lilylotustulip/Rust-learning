fn main() {
    let item_name = String::from("laptop");
    let mut quantity:i32 = 5;
    let mut location:&str = "Shelf A";

    println!("item name : {} | location: {}", item_name, location);

    restock(&mut quantity, &mut location);
    println!("quantity: {}, location: {}", quantity, location);

    let archive = decommition(item_name);
    println!("archive: {}", archive);

    // because item_name was passed to decommition() main no longer owns it so this will result an error.
    //println!("{}", item_name); 

fn restock(quantity:&mut i32, location:&mut &str) {
    let batch:i32 = 3;
    *quantity += batch;
    *location = "Warehouse B";
}

fn decommition(item_name:String) ->String {
    format!("item name: {}", item_name)
}