fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2; // declared inside a scope,  the output should be 6 from the last x variable in the outside scope
        println!("the value of x in the inner scope is: {x}");
    }
    println!("the value of x is: {x}"); // this will take the last x variable in the same scope.
                                        // the output should be 6 because the last assigned value (6) overshadows the previous ones
}
