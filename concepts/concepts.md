# concept of variable and mutability
default type of variable is immutable:
see [example1](./playground/var_and_mut_1.rs)
```text
error[E0384]: cannot assign twice to immutable variable `x`
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
```

but once you add the mut keyword `mut`
see [example2](./playground/var_and_mut_2.rs)
```text
[INFO]: default type of value is immutable: 
current x is: 5
now can change value of x(mutable), now: 6
```
[const variable](./playground/var_and_const_1.rs)

when you use `let` keyword and name the same variable, the first declare variable is being shallow.
[shadowing_1](./playground/var_and_shadow_1.rs)

notice that you should use `let` twice. cause x is immutable.
[shadowing_2](./playground/var_and_shadow_2.rs)
```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> var_and_shadow_2.rs:3:5
  |
2 |     let x = 6;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     x = x - 1;
  |     ^^^^^^^^^ cannot assign twice to immutable variable

error: aborting due to 1 previous error
```

# datatype
see [datatype](./playground/datatype.rs)

# function
see [function](./playground/function.rs)

# comments
```rust
// single line
this line not work.
fn main(){
  /**
   * multi line
   * hey! this line is still commented, it works!.
   */
}
```

# control flow 
[control flow](./playground/control_flow.rs)
