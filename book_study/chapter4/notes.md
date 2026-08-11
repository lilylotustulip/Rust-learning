# OWNERSHIP
Ownership is a set of rules that manages how any rust programs is compiled because it manages its memory.
to understand how a value behaves you have to know whether its on the stack or the heap.
the stack and the heap are available memory spaces to use when running the code.
the stack is like short-term memory, it is fixed in size, therefore if you are unsure about the size of the data use the heap, the heap returns a pointer as the adress of the location it found after being requested to be used.

## RULES
Keep these rules about ownership in mind:
* **each value in Rust has an owner**
* **there can be only one owner at a time**
* **when the owner goes out of scope, the value will be dropped**

### EXAMPLE
let's take this peice of code as an example:

```
fn main () {
    let username = String::from("cookie");
    let clearance_level:i32 = 15;
    let badge = generate_badge(username, clearance_level);
    println!("the badge is,  {badge} ");
    // println!("the username is {username}");
    
}


fn generate_badge(username:String, clearance_level:i32) ->String{
    format!("Username: {}, level: {}", username, clearance_level)
}
```

* **Passing Values:** when we passed `username` to the `generate_badge` function, the ownership of it is moved out of main to `generate_badge`, whereby `main()` loses ownership of `username`.

* **Function Scope:** after passing the value of `username` to the `generate_badge` function, this function has now ownership of it, and after this function finishes excecuting, not only `username` but **any** other variable inside it, that isn't returned, gets dropped.

line 21 will throw a compiler error, because the main function has lost ownership of `username` when it was passed to `generate_badge` function, so to make this line actually work you need to clone or borrow.

*we haven't study this yet so we don't know how it works exactly*

## MEMORY AND ALLOCATION
in rust there is an automatic function that is called whenever the code hits the end of a scope in the compile time. this drop function drops all values, and clean up the heap memory for those variables, so after the scope ends the value doesn't have an owner so its dropped. 
in more complicated situations there may be some unexpected compilor errors.

### VARIABLE AND DATA INTERATING WITH MOVE
when assigning a value of a first variable and that variable as a value to a second one, you bind them, which happens simply with integers and simple data types, however when working with String it becomes a bit tricky. 
A String is made up of 3 parts:
pointer which points to the data on the heap.
length in bytes
And capacity in bytes too.

When you add that second variable, you copy the String parts but not the data on the heap, the pointer is pointing at. so because the 2 variables are now pointing to the same database, when the scope ends rust will try to drop them both at a time, which will cause a bug, so after the second variable is bind to the first one (same value) rust ignores the first.
With that being said, when the scope ends the second variable will be cleaned up, but the first won't because it doesn't even exist anymore. 