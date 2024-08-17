fn main() {
    let x = 5;
    println!("[INFO]: default type of value is immutable: ");
    println!("current x is {x}");
    x = 6;
    println!("cann't change value of x, still: {x}");
    /**
     * error[E0384]: cannot assign twice to immutable variable `x`
        --> var_and_mut_1.rs:5:5
        |
        2 |     let x = 5;
        |         -
        |         |
        |         first assignment to `x`
        |         help: consider making this binding mutable: `mut x`
        ...
        5 |     x = 6;
        |     ^^^^^ cannot assign twice to immutable variable

        error: aborting due to 1 previous error
     */
}