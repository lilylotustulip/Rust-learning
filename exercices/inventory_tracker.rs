fn main() {

    // create an array with a fixed number of integers.
    // do mut to make changes later.
    let mut inventory: [i32; 5] = [7, 4, 9, 10, 14];
    
    // print the array and then the item at the index of 2.
    // we use {:?} to print the whole array and the array_name[index] for the index.
    println! ("the inventory is: {:?} and the third item is: {}", inventory, inventory[2]);

    inventory[4] = 21;
    println!("the updated version of the inventory is now: {:?}", inventory);
}