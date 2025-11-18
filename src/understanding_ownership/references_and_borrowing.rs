#[allow(dead_code)]

pub fn main() {
    reference_and_borrowing();
    mutable_references();
    mutable_references_limited();
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
    println!("r2 is {r2}");
}
