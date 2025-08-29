#[allow(dead_code, unused_variables)]
pub fn main() {
    // Error: consider giving `guess` an explicit type
    // let guess = "42".parse().expect("Not a number!");
    // It works!
    // let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar (标量)
    // Basic types: Integer, Float, Boolean, String

    // Integer
    // | Length  | Signed | Unsigned |
    // | ------- | ------ | -------- |
    // | 8-bit   | i8     | u8       |
    // | 16-bit  | i16    | u16      |
    // | 32-bit  | i32    | u32      |
    // | 64-bit  | i64    | u64      |
    // | 128-bit | i128   | u128     |
    // | arch    | isize  | usize    |
    let explicit_type_int: u8 = 0;
    let prefixed_type_int = 0u8;

    let normal_u8_int: u8 = 255;
    // Integer Overflow
    // let overflowing_u8_int: u8 = 256; // Error: literal out of range for `u8`

    // Floating-Point
    // Default is f64, f64 as fast as f32, and both are signed.
    // Stand for IEEE-754.
    let double_floating_x = 2.0; // f64
    let single_floating_y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let true_bool = true;
    let false_bool: bool = false; // with explicit type annotation

    // The Character
    // The size of char is 4 bytes and also represents a Unicode scalar value.
    let implicit_char = 'z';
    let explicit_char: char = 'z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types
    // tuple (元组) and array (数组)
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // pattern matching and destructuring
    let (tup_x, tup_y, tup_z) = tup;
    println!("The value of tup_x is: {tup_x}");
    // access a tuple element directly by using a period (.)
    let tup_index_1 = tup.1;
    println!("The value of tup_index_1 is: {tup_index_1}");
    // or use the underscore(_) to placeholder
    let (_, _, tup_only_z) = tup;
    println!("The value of tup_only_z is: {tup_only_z}");
    // The tuple without any values has a special name, "unit".
    let unit_tup = ();
    // Array
    // arrays have a fixed length, and every element of an array must have the same type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // arrays allocated on the stack rather than heap.
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // the same value for each element [initial-value; length]
    let same_elements_arr = [3; 5];
    // same as writing this:
    let same_elements_arr2 = [3, 3, 3, 3, 3];
    // An array is a single chunk of memory of a known,
    // fixed size that can be allocated on the stack.
    let first_element = arr[0];
    let second_element = arr[1];
    println!("The value of first_element is: {first_element}");
    // let tenth_element = arr[10]; // Error: index out of bounds: the length is 5 but the index is 10
    
}
