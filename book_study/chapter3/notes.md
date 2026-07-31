# DATA TYPE
Rust is a statically typed language, thus the compiler needs to know all variable's type. 
There is a datatype subset called **scalar** and there is **compound**.

## SCALAR TYPE
The scalar type epresents a **single** value. 
These can be integers, bouleans or even characters.

### INTEGERS
Integers can be signed or unsigned.
Signed means it can either be negative or positive, and unsigned is basically with no sign which makes it default to "positive" only. **u** indicates unsigned while **i** indicates signed.
signed can store numbers from -128 to 127, meanwhile the unsigned can store numbers from 0 to 255.
When you see u32 for example know that its unsigned, and the number, _32 in this case_, is simply the number of bits.

* integer overflow: when you try to change or work with a value over the range that u chose, an integer overflow happens. usually the compilor sends a "panic!" error. But when compiling with the release mode, a wrapping happends.
This does somewhat handles the error, which means no compilor error is thrown at you again, but it usually does do what you want it to do. For instance, the max range is 255 and you have a value with 257, that value that is over the max range, _in this case is 257_ becomes 1.

### FLOATING-POINT NUMBERS
these are decimal numbers, there are 2 types, the **f64** and **f32**. 
**f64** is the default one so if you ever want to use f32 u have to specify it.
example: 
        `fn main() {
            let x = 4.0; // f64

            let y: f32 = 2.0; // f32
        }`
               
### BOULEANS
bouleans, are one byte in sized. they have 2 values, true of false, and is referred to as bool.
example:
        `fn main() {
            let x = true;

            let y: bool = false;
        }`

### CHARACTERS
characters are 4 bytes in size. and specified with single quotes ''.
they support, emojies, letters, and far more.
example:
        `fn main() {
            let a = 'a';
            let star = '✨️';
        }`

## COMPOUND    
unlike the scalar type, compound type can group multiple values into a single type, such as tuples and arrays.

### TUPLE
a tuple is a way of grouping different values with different types into a single compound type.
Tuples are fixed, once declared they cannot be edited.
If ever you want a single value or so out of the tuple, you can destructure it using pattern matching.
example:
        `fn main(){
            let tup = (200, 5, 2.1);
            let (a, b, c) = tup;

            println!("the value of a is {a}");
        }`

You can also access a value within a tuple using a period + the index of the value you want.
A tuple without any value is called a unit. Experessions return the unit value if ever they return nothing else.

### ARRAYS
Arrays are an other type of grouping different values. Arrays only accepts values of the same type and with a fixed length.
An Array's type for example is written this way:
`let a: [i32; 3] = [1, 2, 3];`
the i32, is the type of each value whithin the array, and 3 is the amount of values that this array gonna have.
Arrays can be accessed by indexing, for example to access one in the 'a' array, you do `let num = a[1];` and this way you have assigned 2 to num.

# FUNCTIONS
Functions in rust are declared by the fn keyword,  followed by the name of the fonction which conventionally uses the snake case (all letters lowercase and words are separated with an underscore), and then a set of parentheses.
Then there is the curly brackets which tells rust where the body code begins and ends.
we have seen an example of that in all previous programs, which was the main function.
`fn main() {
    println!("hello, world!);
}
`
## PARAMETERS
parameters are some values you put inside the parentheses, they are part of the function's signature, type declaration is required and multiple parameteres are seperated with commas.
example:
`fn main() {
    temperature(20, 'C');
}
fn temperature (value: u32, temp_type: char) {
    println!("the temperature is: {value}{temp_type}");
}`

## STATEMENT AND EXPRESSIONS
function bodies are made of expressions and statements. So what is an expression? and what is a statement?
* **Expressions** return a value, and they do not end with a semicolon, they make up most of the cod we write in rust because Rust is an expression-based language.

* **Statements** perform an action, but do not return any value. Statements do end with semicolons.

## FUNCTIONS AND RETURN VALUES
functions return values, we declare their type after an arrow (->).
you can assign a function to a variable using its call and the name of the variable.
example:
`fn main() {
    let x = number();
    println!("the number represents: {x}");
}
fn number() -> i32 {
    2
}`
keep in mind, if you want to return a value dont end the line with a semicolon, this will make the expression a statement, leading to an error.

# COMMENTS
comment are ignored by the compiler. They are used for pseudo-code, or code explanation =, to make it easier for the reader.  a comment start by 2 slashes like so : "//" and end in the end of the line.
if you want comments in more than a line all you have to do is add 2 slashes at te beginning of each line.
example:
`// this is a comment`
