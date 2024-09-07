fn main() {
    let _s = String::from("hola!");
    let mut _mut_s = String::from("Hola");
    println!("[INFO]: {}", calc(&_s));
    /*
    error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
        --> reference.rs:12:5
        |
    12 |     s.push_str(" hola!");
        |     ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        |
    help: consider changing this to be a mutable reference
        |
    11 | fn modify(s: &mut String) {
        |               +++

    error: aborting due to 1 previous error
    */
    // modify(&_s);
    modify_mut(&mut _mut_s);
    println!("[INFO]: _mut_s: {}", _mut_s);

    let mut _base_s = String::from("hello");
    let _r1 = &mut _base_s;
    let _r2 = &mut _base_s;
    // error
/*
error[E0499]: cannot borrow `_base_s` as mutable more than once at a time
  --> reference.rs:25:15
   |
24 |     let _r1 = &mut _base_s;
   |               ------------ first mutable borrow occurs here
25 |     let _r2 = &mut _base_s;
   |               ^^^^^^^^^^^^ second mutable borrow occurs here
26 |     println!("[INFO]: {}, {}", _r1, _r2);
   |                                --- first borrow later used here

error: aborting due to 1 previous error
*/
    // println!("[INFO]: {}, {}", _r1, _r2);
    
}

fn calc(s: &String)-> usize {
    return s.len();
}
// fn modify(s: &String) {
//     s.push_str(" hola!");
// }
fn modify_mut(s: &mut String){
    s.push_str(" hola!");
}