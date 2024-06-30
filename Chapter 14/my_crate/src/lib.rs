// Listing 14-2: Documentation for the my_crate crate as a whole
//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

// Listing 14-1: A documentation comment for a function
/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}