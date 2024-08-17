fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    // let _42 = "42".parse().expect("Not a number!"); error
    // decimal 42_000 [valid]
    let _42: u32 = "42".parse().expect("Not a number!");

    // default type of float point is fp64
    let _default_fp = 4.2;
    let _fp32: f32 = 4.2; 
    print!("default fp datatype is: ");
    print_type_of(&_default_fp);
    print!("select fp datatype is: ");
    print_type_of(&_fp32);

    // number operation
    let _sum = 1 + 2;
    let _diff = 4.3 - 2.3;
    let _mul = 2 * 2;
    let _div = 4 / 2;
    let _mod = 4 % 2;

    // bool
    let _b: bool = false;
    
    // charactor
    let _ascii = 'c';
    let _emoji: char = '🧐';
    println!("emoji is supported in rust: {_emoji}.");

    // tuple
    let _tup = (1, 2, 3);
    let _advance_tup: (f32, f64, u32, u8) = (1.1, 2.2, 3, 4);
    let (_a, _b, _c, _d) = _advance_tup;
    println!("value of a is {_a}");
    let _first_of_advtup = _advance_tup;
    println!("first value of advance tuple is: {}", _advance_tup.0);

    // array
    let _arr_demo = [1, 2, 3];
    let _arr_u32: [u32; 3] = [1, 2, 3];
    let _arr_init_with_all_value = [3; 5]; // [3, 3, 3, 3, 3];
}