// ownership in function.

fn main() {
    let x = 5;
    foo(x);
    println!("[INFO]: integer in main: {}", x);
    let _str = String::from("guck");
    take_ownership_foo(_str);
/*
error[E0382]: borrow of moved value: `_str`
  --> ownership_2.rs:9:54
   |
7  |     let _str = String::from("guck");
   |         ---- move occurs because `_str` has type `String`, which does not implement the `Copy` trait
8  |     take_ownership_foo(_str);
   |                        ---- value moved here
9  |     println!("[INFO]: str canno tbe acccess here{}", _str);
   |                                                      ^^^^ value borrowed here after move
   |
note: consider changing this parameter type in function `take_ownership_foo` to borrow instead if owning the value isn't necessary
  --> ownership_2.rs:16:33
   |
16 | fn take_ownership_foo(some_str: String) {
   |    ------------------           ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
8  |     take_ownership_foo(_str.clone());
   |                            ++++++++

error: aborting due to 1 previous error
    */
    // println!("[INFO]: str canno tbe acccess here{}", _str);
    let _own_from_func = give_ownership_foo();
    println!("[INFO]: {}", _own_from_func);
}

fn foo(some_int: i32){
    println!("[INFO]: pass val in fun: {}", some_int);
}

fn take_ownership_foo(some_str: String) {
    println!("[INFO]: foo take str ownership: {}", some_str);
}

fn give_ownership_foo() -> String {
    let _temp = String::from("export");
    return _temp;
}