#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0 
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);

    // let scale = 2;
    let rect1= Rectangle {
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
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     // area(width1, height1)
    //     // area(rect1)
    //     rect1.area()
    // );

    // if rect1.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rect1.width);
    // }

    // println!("rect1 is {:#?}", rect1);

    // dbg!(&rect1);
    
    let sq = Rectangle::square(3);

    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);
    r.set_width(2);
    Rectangle::set_width(&mut r, 2)
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }