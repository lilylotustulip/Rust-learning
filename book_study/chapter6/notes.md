# what is an enum? 
an enum is a some data of different type (or the same type) grouped together under the same scope.

## whats the difference between enum and struct? 
a struct groups some variables AND demands on them all at once, while in enums u work with one at a time, one variable OR an other. 

## the impl block, what is it? how it works?
the impl block gives life and behavior to a struct/enum. after declaring it u add some function that does an act u choose, and give `self` as a parameter to each one of those functions, `self` refers to the struct or enum within main (after giving values to those variables).