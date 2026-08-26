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