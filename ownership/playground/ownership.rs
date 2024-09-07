fn main() {
    // OK.
    let mut x = 5;
    let y = x;
    println!("[INFO]: x: {}, y: {}.", x, y);
    x = 3;
    println!("[INFO]: x: {}, y: {}.", x, y);
    /*
     *    |
        8  |     let _hello = String::from("hello");
        |         ------ move occurs because `_hello` has type `String`, which does not implement the `Copy` trait
        9  |     let mut _world = _hello;
        |                      ------ value moved here
        10 |     println!("[INFO]: {} {}.", _hello, _world);
        |                                ^^^^^^ value borrowed here after move
        |
     */
    // let _hello = String::from("hello");
    // let _world = _hello;
    // println!("[INFO]: {} {}.", _hello, _world);
    let _hello = String::from("hello");
    let mut _world = _hello.clone();
    _world = String::from("world");
    println!("[INFO]: {}, {}.", _hello, _world);
}