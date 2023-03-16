use std::io;

fn deg_f_to_deg_c(){
    println!("Enter a value in Farenheit.");

    let mut deg_f = String::new();

    io::stdin()
        .read_line(&mut deg_f)
        .expect("Failed to read line.");

    let deg_f: f64 = deg_f.trim().parse().expect("Not a number");

    let deg_c = (deg_f-32.0)*5.0/9.0;

    println!("{deg_f}degF is {deg_c}degC!");
}

fn nth_fib(n: u64) -> u64 {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    for _ in 1..n {
        let next = prev+curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn twelve_days_of_christmas() {
    
    let days = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "FIVE! GOLDEN! RINGS!!!",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in 0..12{
        if day <= 2 {
            let count;
            match day {
                0 => count = "1st",
                1 => count = "2nd",
                2 => count = "3rd",
                _ => break
            }
            println!("On the {count} day of Christmas, my true love sent to me");
        }
        else {
            println!("On the {}th day of Christmas, my true love sent to me", day+1);
        }
        for n in 0..=day{
            println!("{}", days[day-n]);
        }
    }
}

fn main() {
    //deg_f_to_deg_f();
    // for i in 1..=10{
    //     println!("{}", nth_fib(i));
    // }
    twelve_days_of_christmas();
}