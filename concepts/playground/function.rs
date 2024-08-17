fn foo(){
    println!("[INFO]: hey, im in foo function.");
}
fn foo_with_param_int(x: i32){
    println!("[INFO]: parameter: {x}");
}
// notice you should input a &str in this function while call with 
// foo_with_param_int_and_string(1, "hello");
fn foo_with_param_int_and_string(idx: i32, val: &str){
    println!("[INFO]: arr[{idx}] is {val}");
}

// function with return value 
// notice the return value shouldn't end with `;`
fn foo_with_return_right(x: i32) -> i32 {
    x + 1
    // return x + 1; // is also ok
}

// fn foo_with_return_error(x: i32) -> i32 {
//     x + 1;
// }

fn main(){
    println!("[INFO]: in main");
    // notice: "world" datatype is &str
    let _str: &str = "world";
    foo();
    foo_with_param_int(42);
    foo_with_param_int_and_string(1, &_str);

    println!("[INFO]: right foo return with {}", foo_with_return_right(1));

    let _y = {
        let x = 41;
        x + 1 // shouldn't end with ;
    };
    println!("[INFO]: y is {_y}");
}