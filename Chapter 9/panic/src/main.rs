use std::net::IpAddr;

fn main() {
    // panic!{"crash and burn"};

    // Listing 9-1: Attempting to access an element beyond 
    //the end of a vector, which will cause a call to panic!
    // let v = vec![1, 2, 3];
    // v[99];

    let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");

}

// Listing 9-13: A Guess type that will only continue with 
// values between 1 and 100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}