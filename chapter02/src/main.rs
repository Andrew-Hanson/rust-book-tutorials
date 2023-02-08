fn main() {
    macro_rules! ignore {
        ($_:expr) => {};
    }

    // this macro deletes this code block so the program can compile.
    ignore!(
    // compile error
    {
        let x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    });
    // x in not marked as mutable,
    // and is not being declared as a shadow variable

/*
    $ cargo run
       Compiling variables v0.1.0 (file:///projects/variables)
    error[E0384]: cannot assign twice to immutable variable `x`
     --> src/main.rs:4:5
      |
    2 |     let x = 5;
      |         -
      |         |
      |         first assignment to `x`
      |         help: consider making this binding mutable: `mut x`
    3 |     println!("The value of x is: {x}");
    4 |     x = 6;
      |     ^^^^^ cannot assign twice to immutable variable

    For more information about this error, try `rustc --explain E0384`.
    error: could not compile `variables` due to previous error
*/









}
