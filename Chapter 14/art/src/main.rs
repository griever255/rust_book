// Listing 14-4: A crate using the art crate’s items with its
// internal structure exported
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// Listing 14-6: A program using the re-exported items from
// the art crate
use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}