fn main(){
    let x = 6;
    let x = x - 1;
    { // start life circle
        // inner scope
        // notice: every time you use let variable you new a new variable.
        let x = x * 2;
        println!("inner x: {x}");
    } // end life circle

    println!("outter x: {x}");
}