use std::io;
use std::io::Write;

fn main() {
    temp_converter()
    fibonacci()
    sing()
}

// Convert temperatures between Fahrenheit and Celsius.
fn temp_converter() -> () {
    let mut degf: Option<f64> = None;
    println!("Convert a number from degrees Farenheit to Celsius");
    while degf.is_none() {
        print!("Enter a number in degF: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        degf = match input.trim().parse() {
            Ok(degf) => Some(degf),
            Err(_) => continue,
        };
    }
    let degc: f64 = (degf.unwrap() - 32.) * 5. / 9.;
    println!("That number in degC is: {:.0}", degc);
}

// Generate the nth Fibonacci number.
fn fibonacci() -> () {
    let mut n: Option<i128> = None;
    println!("Compute the nth Fibonacci number!");
    while n.is_none() {
        print!("Enter the index of the fibonnaci number to compute: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        n = match input.trim().parse() {
            Ok(n) => Some(n),
            Err(_) => continue,
        };
    }
    let n = n.unwrap();

    let mut result: i128 = 1;
    let mut last: i128 = 1;
    let mut last_two: i128 = 0;
    for _ in 1..n {
        result = last + last_two;
        last_two = last;
        last = result;
    }
    println!("The {n}th fibonacci number is: {result}");
}

// Print the lyrics to the Christmas carol 
// “The Twelve Days of Christmas,” 
// taking advantage of the repetition in the song.
fn sing() -> () {
    const lyric_by_day: [&str; 12] = [
        "And a partridge in a pear tree!",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    const ordinal: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelve",
    ];

    let mut day_index;
    for i in (0..12).rev() {
        day_index = 11 - i;
        println!("On the {} day of Christmas,", ordinal[day_index]);
        println!("my true love gave to me,");
        for j in (0..=day_index).rev() {
            if day_index == 0 {
                println!("A partridge in a pear tree!")
            } else {
                println!("{}", lyric_by_day[j]);
            }
        }
        if day_index != 11 {
            println!("");
        }
    }
}
