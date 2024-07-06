use std::ops::Add;

// 
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Listing 19-14: Implementing the Add trait to overload the + 
// operator for Point instances
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Listing 19-15: Implementing the Add trait on Millimeters to
// add Millimeters to Meters
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Listing 19-16: Two traits are defined to have a fly method and are
// implemented on the Human type, and a fly method is implemented on
// Human directly
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Listing 19-19: A trait with an associated function and a type with an
// associated function of the same name that also implements the trait
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Listing 19-22: Implementing the OutlinePrint trait that
// requires the functionality from Display
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Listing 19-23: Creating a Wrapper type around Vec<String> to
// implement Display
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Listing 19-17: Calling fly on an instance of Human
    let person = Human;
    person.fly();

    // Listing 19-18: Specifying which trait’s fly method we want to
    // call
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    
    println!("A baby dog is called a {}", Dog::baby_name());

    // Listing 19-21: Using fully qualified syntax to specify that we
    // want to call the baby_name function from the Animal trait as
    // implemented on Dog
    println!("A baby dog is called {}", <Dog as Animal>::baby_name());

    Point { x: 3, y: 3 }.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}