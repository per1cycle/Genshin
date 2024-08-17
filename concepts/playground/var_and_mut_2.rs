fn main() {
   let mut x = 5;
   println!("[INFO]: default type of value is immutable: ");
   println!("current x is: {x}");
   x = 6;
   println!("now can change value of x(mutable), now: {}", x);
}
