# STRUCTS DEFINITION
a struct is similar to a tuple. it can hold different type of data. 
it is defined by using the keyword `struct`, followed by the corresponding name (a name you choose, preferably related to what this struct conatians) and then curly brackes, within those curly brackets there is the name of what we call fields then : and then the data type. 
so its something like this: 
```
struct ID
   {
    name:String,
    birthday:i32,
    }
```
to create an instance of a struct you declare a varibale that will hold the struct and just rewrite it but by adding the actual values of the variables that the struct contains.
there is also a field init shorthand, where you can just write lets say `name` instead of `name: name`.

## STRUCT UPDATE SYNTAX
a struct updated syntax is when u already have a struct and want to build an other one similar to it by changing only few varibales, u change whichever variables u want to change and do `..struct1` so all other variables are within the second struct. 

## TUPLE STRUCT
tuple struct which are like tuples, u only get to name the struct itself and no fields u go with the data types dirrectly. you access them by using indexes, for example `.0` or `.1`

## UNIT LIKE STRUCT
a struct with no data yet u do it by writing the struct keyword followed by the struct name and then semicolons.
no parentheses, no curly brackets. 

# METHODS
## DEFINTION
methods are defined based on some struct (or enum.. we havent study that yet) they are similar to function. methods are defined using the fn keyword. all method start with a parameter at least. which is the self parameter, it can be a reference like `&self` or even mutable so you can change it and edit it within the method. this self parameter is like an instance, where you take the values and work with them whithin the method that has this `self` as a parameter. a method is a type of associated function, which means it goes whithin a impl block. an impl block like its name sais, a block where methods, and associated functions goes and structs are used with those methods.

```
#[derive(Debug)] 
struct Rectangle { // declaring the main struct
    // some variables within the struct
    width: u32,    
    height: u32,
}

impl Rectangle { // impl block which will contain the method that will use the struct
    fn width(&self) -> bool { // method called width with a &self parameter
                              // which means it will go to the instance of the struct to take the values it needs 
                              // and work with them here, and later returning that value.

        self.width > 0        // here the self which is the parameter that act like an instance calls for a variable
                              // from the main struct and work with it.
    }
}

fn main() {
    let rect1 = Rectangle { // the struct instance
        // the values that &self is pointing to, and main still has ownership of them
        width: 30,
        height: 50,
    };

    if rect1.width() { // method call
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```


## METHOD CALL SYNTAX
when calling a method you do the structname.methodname(). you dont need to do the referencing mutabilty or even dereferencing symboles here because rust handles that automatically.

## NON-METHOD ASSOCIATED FUNCTION
a non-method associated function or a construcor, is when u make up some function and use it for a struct, when you have no instance yet. with that you dont actually have a method, but still it is an assosiated function. the syntax is pretty commun to a frequently used line: `name = String::from("name");`