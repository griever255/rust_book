// // Listing 10-6: A Point<T> struct that
// // holds x and y values of type T
// struct Point<T> {
// //struct Point<T, U> {
//     x: T,

//     // Listing 10-8: A Point<T, U> generic over two 
//     // types so that x and y can be values of different types
//     // y: U,
//     y: T,
// }

// // Listing 10-9: Implementing a method named x on the Point<T>
// // struct  that will return a reference to the x field of type T
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// Listing 10-11: A method that uses generic types 
// different from its struct’s definition
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


// Listing 10-10: An impl block that only applies to a struct
//  with a particular concrete type for the generic type
// parameter T
// impl Point<f32> {
//     fn distance_to_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


fn main() {
    // Listing 10-1: Finding the largest number in a 
    // list of numbers
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // Listing 10-3: Abstracted code to find the 
    // largest number in two lists
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    // Listing 10-2: Code to find the largest number in 
    // two lists of numbers
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    let result = largest(&number_list);
    println!("The largest number is {result}");

    // Listing 10-4: Two functions that differ only in their 
    // names and the types in their signatures
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    // Listing 10-6: A Point<T> struct that
    // holds x and y values of type T
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // Listing 10-8: A Point<T, U> generic over two types so 
    // that x and y can be values of different types
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // Listing 10-11: A method that uses generic types 
    // different from its struct’s definition
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Listing 10-3: Abstracted code to find the 
// largest number in two lists
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Listing 10-4: Two functions that differ only in their 
// names and the types in their signatures
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}