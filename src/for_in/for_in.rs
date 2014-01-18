/**
 * for_in.rs : Example of the rust for loop.
 *
 * The Rust 'for' loop is very similar to the 'for' in Python. That is to say
 * It is rather different than the C implementation and works through iterating
 * though a fixed list of values.
 *
 * The syntax is "for iter_var in <Iter>" which is valid for any function
 * implementing the iterator trait.
 *
 * See : http://static.rust-lang.org/doc/master/std/iter/index.html
 * for a more involved explanation.
 **/

fn main() {
    // range(a,b) returns an iterator with integer values, i, from (a <= i < b).
    for i in range(1,10){
        println!("i is now : {}",i);
    }
}
