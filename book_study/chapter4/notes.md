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

## Understanding Rust Memory: Moves, Clones, and the Copy Trait
in rust there is an automatic function that is called whenever the code hits the end of a scope in the runtime. this drop function drops all values, and clean up the heap memory for those variables, so after the scope ends the value doesn't have an owner so its dropped. 
in more complicated situations there may be some unexpected compilor errors.

when assigning a value of a first variable and that variable as a value to a second one, you bind them, which happens simply with integers and simple data types, however when working with String it becomes a bit tricky. 
A String is made up of 3 parts:
pointer which points to the data on the heap.
length in bytes
And capacity in bytes too.

When you add that second variable, you copy the String parts but not the data on the heap, the pointer is pointing at. so because the 2 variables are now pointing to the same database, when the scope ends rust will try to drop them both at a time, which will cause a bug, so after the second variable is bind to the first one (same value) rust ignores the first.
With that being said, when the scope ends the second variable will be cleaned up, but the first won't because it doesn't even exist anymore. 

moreover when you get a variable called for example s1 assign a value to it and get an other variable also called s1 with an other value, different from the first one, the first gets replaced by the second.

There is a method to specifically copy the data on the heap which is `.clone()`.

There is also an annotation that works with some data types not all, like integers, characters and booleans, because they are fixed sized and stored on the stack. it is the `copy` trait when a variable has "copy" it makes it available after assigning it to an other variable, it doesnt move it, but it makes a sort of copy of it. 

When passing a variable to an other function, other than where it was declared you should also move its owner, you store it in the parameters of this function and then you may use it. 
Also after you are done with some function and want to return its value, you should store it within a variable in `main`,or the calling function, so you can get use of this returned value. 

## REFERENCES AND BORROWING

a reference is like a pointer, but the reference points exactly to the value, you refer to something but dont take ownership of it.
Just like variables, references are also immutable by default, and when they are immutable u can refer to the same thing as many times you want, however if you make it mutable you can only refer to that thing once at a time. and you cannot do a mutable reference if there is an immutable reference, that refers to the same value, it gives an error.
However, there is some exception. 

### example: 
```fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // works because immutable reference
    let r2 = &s; // same idea
    println!("{r1} and {r2}"); 


    let r3 = &mut s; 
    println!("{r3}"); // works too because of the previous println!

    println!("{r2}"); // error!

}
```

You can borrow a value from a function to an other by using a reference too. you dont move ownership of it, you dont even have to return it after being used in an other function because the function in which it was called has ownership of it already. 

There is dangle pointer which is a pointer that refers to some location frees some of the memory while preserving a pointer to that memory, Rust prevents these at compile time, because they cause security vulnerabilities.
