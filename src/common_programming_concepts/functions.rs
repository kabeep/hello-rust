#[allow(dead_code, unused_variables)]
pub fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5, 'h');

    // Statements & Expressions
    let statement_var = "Expressions";
    // let x = (let y = 6); // This is wrong, different from C / Ruby / JS

    // Block & Expression
    let block_statement_var = {
        let x = 3;
        x + 1
    };
    println!("block_statement_var is: {}", block_statement_var);

    let five = return_five();
    println!("five is: {}", five);

    let plus_res = plus_one(five);
    println!("plus res is: {}", plus_res);
}

// Rust doesn’t care where functions define,
// only that they’re defined somewhere in a scope that can be seen by the caller.
fn another_function() {
    println!("Another function!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
