fn main(){
    let _key = 15;
    if _key < 1 {
        println!("[INFO]: less than 1");
    }else if _key == 15 {
        println!("[INFO]: correct.");
    }else {
        println!("[ERROR]: val error");
    }
    let _cool: bool = false;
    let _val = if _cool { 42 } else { -1 };
    println!("[INFO]: cool value is {_val}");

    const LOOP_TIME: u32 = 5;
    let mut _loop_time = 0;
    loop {
        _loop_time = _loop_time + 1;
        if _loop_time > LOOP_TIME {
            break;
        }
        println!("[INFO]: In loop {_loop_time}");
    }
    
    // you can make break loop more clearly.
    'cool_loop: loop {
        println!("[INFO]: cool_loop {_loop_time}");
        'not_that_cool_loop: loop {
            _loop_time = _loop_time + 1;
            println!("[INFO]: not_that_cool_loop {_loop_time}");
            if _loop_time % 5 == 0 {
                break 'not_that_cool_loop;
            }
        }
        _loop_time = _loop_time + 1;
        
        if _loop_time > 20 {
            break 'cool_loop;
        }
    }

    // while/for loop 
    
    let mut _iter = 0;
    let _one = [1, 2, 3, 4, 5];
    while _iter < 5 {
        println!("[INFO]: _iter: {}, arr[{}] = {}", _iter, _iter, _one[_iter]);
        _iter += 1;
    }
    for elem in _one {
        println!("[INFO]: elem: {}", elem);
    }
    // [1, 4)
    for iter in (1.. 4).rev() {
        println!("[INFO]: rev iter: {}", _one[iter]);
    }
}