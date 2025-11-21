use std::rc::Rc;

#[allow(unused_imports)]
pub fn main() {
    // Rust will always reject an unsafe program.
    // But sometimes, Rust will also reject a safe program.
    fixing_an_unsafe_program();
}

fn fixing_an_unsafe_program() {
    returning_a_reference_to_the_stack();
    not_enough_permissions();
}

fn returning_a_reference_to_the_stack() {
    /*
     * Here, the issue is with the lifetime of the referred data.
     * If we want to pass around a reference to a string, we have to make sure that underlying string lives long enough.
     */
    // fn return_a_string() -> &String {
    //     let s = String::from("Hello world");
    //     &s // returning a reference to the stack
    // }
    // Here are four ways we can extend the lifetime of the string.
    // Which strategy is most appropriate will depend on our application.
    // 1. Move ownership of the string out of the function.
    fn return_a_string_a() -> String {
        let s = String::from("Hello world");
        s
    }
    // 2. Return a string literal, which lives forever (indicated by 'static)
    fn return_a_string_b() -> &'static str {
        "Hello world"
    }
    /**
     * 3. Defer borrow-checking to runtime by using garbage collection.
     * In short, Rc::clone only clones a pointer to `s` and not the data itself. At runtime, the
     * `Rc` check when the last `Rc` pointing to data has been dropped, and then deallocates the data.
     * @see [reference-counted pointer](https://doc.rust-lang.org/std/rc/index.html)
     */
    fn return_a_string_c() -> Rc<String> {
        let s = Rc::new(String::from("Hello world"));
        Rc::clone(&s)
    }
    // 4. Have the caller provide a `slot` to put the string using a mutable reference.
    fn return_a_string_d(output: &mut String) {
        output.replace_range(.., "Hello world");
    }

    let mut s = String::from("Hello");
    println!("s is {s}");
    
    let s1 = return_a_string_a();
    println!("s1 is {s1}");
    
    let s2 = return_a_string_b();
    println!("s2 is {s2}");
    
    let s3 = return_a_string_c();
    println!("s3 is {s3}");

    return_a_string_d(&mut s);
    println!("s is {s} now");
}

fn not_enough_permissions() {
    
}
