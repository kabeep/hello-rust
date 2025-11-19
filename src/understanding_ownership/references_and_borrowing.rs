#[allow(dead_code)]
pub fn main() {
    reference_and_borrowing();
    mutable_references();
    mutable_references_limited();
    dangling_references();

    // The rules for summarizing `Reference` are:
    // - There can be either only one mutable reference
    //      or multiple immutable references at any given time.
    // - References must always be valid.
}

fn reference_and_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // reference
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is reference for s1 // s ptr refer to s1 ptr and s1 ptr refer to data in heap memory
    s.len() // s does not have ownership for s1, so there is no need to hand over ownership
} // s was move out, and it does not call the drop function

fn mutable_references() { // the concept of borrowing belongs to reference
    let mut s = String::from("hello"); // the borrowing is not immutable, s must be mutable
    change(&mut s); // use &mut here
    println!("s is {s}");
}

fn change(some_string: &mut String) { // use &mut here also, borrowing is mutable again
    some_string.push_str(", world");
}

fn mutable_references_limited() {
    let mut s = String::from("hello");
    let r1 = &mut s; // here is use the reference from s, and borrowed ownership to r1
    // let r2 = &mut s; // so we cannot use a new reference to borrowing s, here will be thrown an error
    println!("r1 is {r1}"); // r1 leaves the scope here

    let r2 = &mut s; // so we can create a new reference perfectly
    println!("r2 is {r2}"); // then use r2

    // although variable in Rust is different from most languages,
    // they do a good job of avoiding data race when compile

    // data race will occur in the following situations:
    // - two or more ptr access the same data simultaneously
    // - at least one ptr is used to write data
    // - no mechanism to synchronize data access

    {
        let r3 = &mut s;
        println!("r3 is {r3}");
    } // r3 was move out

    let r4 = &mut s; // so we can create a new reference here
    println!("r4 is {r4}");

    let r5 = &s; // no problem
    let r6 = &s; // no problem because it is an immutable reference
    // big problem because at least one ptr is used to write data
    // let r7 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("r5 is {r5}, r6 is {r6}");

    let r8 = &s; // no problem
    let r9 = &s; // no problem
    println!("r8 is {r8}, r9 is {r9}"); // move out scope for r8 and r9
    let r10 = &mut s; // no problem because their scopes are not overlap
    println!("r10 is {r10}");
}

fn dangling_references() {
    // the Dangling Reference mean that when memory is released a ptr to it is retained, 
    // this is called a dangling pointer

    // let reference_to_nothing = dangle();
    // fn dangle() -> &String { // dangle return a reference of String
    //     let s = String::from("hello"); // s is a new String
    //     &s // return a reference of String that type contains a borrowed value
    // } // s dropped here and release its memory but reference `&s` is retained， there is no value for it to be borrowed from

    // the correct approach is:
    let reference_to_string = no_dangle();
    println!("reference_to_string is {reference_to_string}");
    fn no_dangle() -> String {
        let s = String::from("hello");
        s // the ownership of `s` will move out and borrow to `reference_to_string`
    }
}
