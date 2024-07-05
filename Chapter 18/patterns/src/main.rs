fn main() {
    // Listing 18-1: Mixing if let, else if, else if let, and else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // Listing 18-2: Using a while let loop to print values for as
    // long as stack.pop() returns Some
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // Listing 18-3: Using a pattern in a for loop to destructure a
    // tuple
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // Listing 18-4: Using a pattern to destructure a tuple and create
    // three variables at once
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    let some_option_value: Option<i32> = Some(2);
    // Listing 18-8: Attempting to use a refutable pattern with let
    // Listing 18-9: Using if let and a block with refutable patterns
    // instead of let
    if let Some(x) = some_option_value {
        println!("{x}");
    }

    // Listing 18-10: Attempting to use an irrefutable pattern with
    // if let
    if let x = 5 {
        println!("{x}");
    }

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Listing 18-11: A match expression with an arm that introduces a
    // shadowed variable y
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y:?}");


    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Listing 18-12: Destructuring a struct’s fields into separate
    // variables
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Listing 18-13: Destructuring struct fields using struct field
    // shorthand
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Listing 18-14: Destructuring and matching literal values in one
    // pattern
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("on neither axis: ({x}, {y})"),
    }

    // Listing 18-15: Destructuring enum variants that hold different
    // kinds of values
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // Listing 18-16: Matching on nested enums
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, value {v}");
        }
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    // Listing 18-18: Using an underscore within patterns that match Some
    // variants when we don’t need to use the value inside the Some
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    // Listing 18-19: Ignoring multiple parts of a tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // Listing 18-20: Starting a variable name with an underscore to
    // avoid getting unused variable warnings
    let _x = 5;
    let y = 10;

    // Listing 18-22: Using an underscore does not bind the value
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    // Listing 18-23: Ignoring all fields of a Point except for x by
    // using ..
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    }

    // Listing 18-24: Matching only the first and last values in a
    // tuple and ignoring all other values
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Listing 18-26: Adding a match guard to a pattern
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Listing 18-27: Using a match guard to test for equality with
    // an outer variable
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Listing 18-28: Combining multiple patterns with a match guard
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // Listing 18-29: Using @ to bind to a value in a pattern while
    // also testing it
    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello { id: id_variable @ 3..=7 } => 
            println!("Found an id in range: {id_variable}"),
        HelloMessage::Hello { id: 10..=12 } =>
            println!("Found an id in another range"),
        HelloMessage::Hello { id } => println!("Found some other id: {id}"),
        
    }

}

enum HelloMessage {
    Hello { id: i32 },
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// Listing 18-6: A function signature uses patterns in the parameters
// Listing 18-17: Using _ in a function signature
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

// Listing 18-7: A function with parameters that destructure a tuple

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: {x}, {y}");
}