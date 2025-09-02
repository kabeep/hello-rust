#[allow(dead_code)]
pub fn main() {
    // `if` Expressions
    let number = 3;

    // Blocks of code associated with the conditions in `if` expressions are
    // sometimes called `arms (分支)`, just like the arms in match expressions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The condition must be a `bool`, Unlike languages such as Ruby and
    // JavaScript 'if (1) {}', Rust will not automatically try to convert
    // non-Boolean types to a Boolean.
    // if number { println!("number was three"); } // Error: expected `bool`, found integer

    // Not equal
    if number != 0 { println!("number was something other than zero"); }

    // Handle Multiple Conditions with `else if`
    let number = 6;
    if number % 4 == 0 { println!("number is divisible by 4"); } else if number % 3 == 0 { println!("number is divisible by 3"); } else if number % 2 == 0 {
        // Rust only executes the block for the first true condition,
        // and once it finds one, it doesn’t even check the rest, even
        // though 6 is divisible by 2
        println!("number is divisible by 2");
    } else { println!("number is not divisible by 4, 3, or 2"); }

    // Using `if` in a `let` statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // The values that have the potential to be results from each arm
    // of the `if` must be the same type
    // let number = if condition { 5 } else { "six" }; // Error: expected integer, found `&str`

    // Loops (循环)
    // Repeating Code with `loop`
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 3 {
            counter = 0;
            break;
        }
        println!("again!");
    }

    // Return Values from Loops
    let result = loop {
        counter += 1;
        if counter == 10 { break counter * 2; };
    };
    println!("The result is {}", result);

    // Loop Labels to Disambiguate Between Multiple Loops
    // If there have loops within loops, `break` and `continue` apply to
    // the innermost loop at that point.
    let mut count = 0;
    'counting_up: loop { // Loop Label `'counting_up`
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with `while`
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with `for`
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 { // same as index < arr.len()
        println!("the value is: {}", arr[index]);
        index += 1;
    }
    // Use `for` instead of it to avoid changed the definition of the array
    // but forgot to update the condition
    for element in arr { // we’ve now increased the safety of the code
        println!("the value is: {}", element);
    }

    // Another benefit of using `for` is that we can use `range`
    for number in (1..4).rev() { // `rev` to reverse the range, (1..4) same as (1..=3)
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
