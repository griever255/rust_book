// Listing 5-1: A User struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // Listing 5-2: Creating an instance of the User struct
    // We use owned Strings because a struct should own
    // all of its data, unless we want to use lifetimes
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Listing 5-3: Changing the value in the email field
    // of a User instance
    user1.email = String::from("anotheremail@example.com");

    // Listing 5-6: Creating a new User instance using one
    // of the values from user1
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // Listing 5-7: Using struct update syntax to set a new email
    // value for a User instance but to use the rest of the values
    // from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Note: We can no longer use user1, username was moved
    };

    // Tuple Structs without Named Fields
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs
    let subject = AlwaysEqual;
}

// Listing 5-4: build_user function that takes an email and username
// and returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Listing 5-5: A build_user function that uses field init
        // shorthand because the username and email parameters
        // have the same name as struct fields
        username,
        email,
        sign_in_count: 1,
    }
}
