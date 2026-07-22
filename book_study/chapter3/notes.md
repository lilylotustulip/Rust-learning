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