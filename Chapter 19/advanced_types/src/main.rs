use std::fmt;
use std::io::Error;
type Result<T> = std::result::Result<T, std::io::Error>;

type Thunk = Box<dyn Fn() + Send + 'static>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
}

// Listing 19-24: Using a long type in many places
// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
//     // --snip--
// }

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//     // --snip--
// }

// Listing 19-25: Introducing a type alias Thunk to reduce repetition
// fn takes_long_type(f: Thunk) {
//     // --snip--
// }

// fn returns_long_type() -> Thunk {
//     // --snip--
// }
