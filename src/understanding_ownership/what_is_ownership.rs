#[allow(dead_code)]
pub fn main() {
    // Ownership is a set of rules that govern how a Rust program manages memory.
    // All programs have to manage the way they use a computer’s memory while running.
    // Some languages have garbage collection that regularly looks for no-longer-used memory
    // as the program runs; in other languages, the programmer must explicitly allocate
    // and free the memory. Rust uses a third approach: memory is managed through a system
    // of ownership with a set of rules that the compiler checks.If any of the rules
    // are violated, the program won’t compile. None of the features of ownership
    // will slow down your program while it’s running.

    {
        let s = "hello";
        println!("s is {}", s);  // use `s` here
    }
    println!("`s` not found in this scope");

    let x = 5;
    let y = x;  // here is a shallow copy instead of move ownership
    println!("x is {}, y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;    // `s2` ptr to heap data of `s1` and release `s1` memory
    println!("`s2` is same as `s1` as {}", s2);
    println!("`s1` borrowed here after move");

    let mut s3 = String::from("hello");
    println!("`s3` is {s3} as mutable variable");
    s3 = String::from("ciao");  // release `hello` memory for heap
    println!("`s3` is {} now", s3);

    let s4 = String::from("hello");
    let s5 = s4.clone();    // shallow copy `trait`
    println!("`s4` is {}, `s5` is {}", s4, s5);

    // Copy trait types
    // - integer, such as `u32`
    // - bool, `true` or `false`
    // - float, such as `f64`
    // - character, `char`
    // - tuple with copy implement, such as (i32, i32), but (i32, String) does not

    ownership_and_functions();
    return_value_and_scoped();
    keep_ownership_and_argument();
}

// region Ownership & Functions
fn ownership_and_functions() {
    let s = String::from("hello"); // enter this scoped
    takes_ownership(s); // move in function

    let x = 5;  // enter this scoped
    makes_copy(x);  // move in function but i32 has copy trait
    println!("x is {x}"); // so we can use x after here
} // `x` move out scoped first, followed by `s`, but `s` was moved out such nothing special

fn takes_ownership(some_string: String) { // some_string enter this scoped
    println!("takes_ownership string of {some_string}");
} // some_string move out this scoped and call fn `drop` to release memory

fn makes_copy(some_integer: i32) { // some_integer enter this scoped
    println!("makes_copy integer of {some_integer}");
} // some_integer move out this scoped, nothing special
// endregion

// region Return Value & Scoped
fn return_value_and_scoped() {
    let s1 = gives_ownership(); // gives_ownership borrowed return value to s1 
    println!("return_value_and_scope s1 is {}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 move in takes_and_gives_back then move to s3
    println!("return_value_and_scope s3 is {}", s3);
} // `s3` dropped first and `s2` was move out so nothing happen, `s1` dropped

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // return some_string and move to call function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // return a_string and move to call function
}

// endregion

// region Keep Ownership & Argument
fn keep_ownership_and_argument() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
