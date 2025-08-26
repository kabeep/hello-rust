#[allow(dead_code)]
pub fn main() {
    // Variable (immutable)
    // let x = 5;
    // println!("The value of x is: {x}");
    // Error: cannot assign twice to immutable variable `x`
    // x = 6;
    // println!("The value of x is: {x}");

    // Variable (mutable)
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = 5;
    let x = x + 1;
    // Scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Variable Type (immutable)
    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");

    // Variable Type (mutable)
    // let mut spaces = "   ";
    // Error: mismatched types
    // spaces = spaces.len();
}
