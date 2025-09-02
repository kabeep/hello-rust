#[allow(dead_code)]
pub fn main() {
    let fahrenheit = convert_fahrenheit_and_celsius(36.9, true);
    let celsius = convert_fahrenheit_and_celsius(98.42, false);
    println!("Body temperature of a human: {fahrenheit} °F / {celsius} °C");

    let ninth_fibonacci = get_fibonacci(9);
    println!("Ninth fibonacci number: {ninth_fibonacci}");

    print_song_lyric();
}

fn convert_fahrenheit_and_celsius(number: f32, is_celsius: bool) -> f32 {
    if is_celsius {
        number * 1.8 + 32.0
    } else {
        (number - 32.0) / 1.8
    }
}

fn get_fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

fn print_song_lyric() {
    println!("The Twelve Days of Christmas:");
    
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas,", ordinals[day]);
        println!("My true love sent to me:");

        for index in (0..=day).rev() {
            match index {
                0 if day > 0 => println!("and {}.", gifts[index]),
                0 => println!("{}.", gifts[index]),
                _ => println!("{},", gifts[index]),
            }
        }

        println!();
    }
}
