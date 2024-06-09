// Example program using structs

// Listing 5-12: Adding the attribute to derive the Debug 
// trait and printing the Rectangle instance using debug formatting
#[derive(Debug)]
// Listing 5-10: Defining a Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Listing 5-13: Defining an area method on the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    // Listing 5-15: Implementing the can_hold method on Rectangle 
    // that takes another Rectangle instance as a parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// // Listing 5-16: Rewriting Listing 5-15 using multiple impl blocks
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// Listing 5-8: Calculating the area of a rectangle
// specified by separate width and height variables
fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // Listing 5-9: Specifying the width and height of the
    // rectangle with a tuple
    // let rect1 = (30, 50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    let scale = 2;
    // Listing 5-10: Defining a Rectangle struct
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    // Listing 5-11: Attempting to print a Rectangle instance
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        // Pass a reference to the struct
        // can reuse rect1 and
        // does not move the fields of rect1
        
        // area(&rect1)

        rect1.area()
    );
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Listing 5-14: Using the as-yet-unwritten can_hold method
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq is equal to: {:#?}", sq);
}

// Listing 5-8: Calculating the area of a rectangle
// specified by separate width and height variables
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Listing 5-9: Specifying the width and height of the
// rectangle with a tuple
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Listing 5-10: Defining a Rectangle struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}