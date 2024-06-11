//Listing 7-1: A front_of_house module 
// containing other modules that then contain functions


pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}