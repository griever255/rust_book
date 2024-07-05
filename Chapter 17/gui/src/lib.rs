// Listing 17-3: Definition of the Draw trait
pub trait Draw {
    fn draw(&self);
}

// Listing 17-4: Definition of the Screen struct with a components
// field holding a vector of trait objects that implement the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Listing 17-5: A run method on Screen that calls the draw method on
// each component
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// // Listing 17-6: An alternate implementation of the Screen struct and
// // its run method using generics and trait bounds
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// Listing 17-7: A Button struct that implements the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}