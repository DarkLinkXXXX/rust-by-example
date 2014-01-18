/**
 * functions.rs : A cursory introduction to rust functions.
 *
 * Functions in Rust are declared by the keyword 'fn'.
 * The basic syntax is:
 *     fn my_function(my_var: int) -> ret_val {
 *         ...
 *         return ret_val;
 *     }
 *
 * For more info see : http://static.rust-lang.org/doc/master/rust.html#functions
 **/

// A simple void function
fn func_1() {
    println!("The void function");
}

// Parameters with no return
fn func_2(x: int, y: f32, z: bool) {
    println!("Parameterized Function");
    println!("  An Integer: {}",x);
    println!("  A  float:   {}",y);
    println!("  A  boolean: {}",z);
}

// With return values
fn func_3(x: int,y: int) -> int {
    println!("Function with return value")
    return x+y;
}

// Main function is always called first in a Rust program.
fn main() {
    func_1();
    func_2(2, 3.14159265, true);
    println!("Value returned : {}",func_3(42, 378));
}
