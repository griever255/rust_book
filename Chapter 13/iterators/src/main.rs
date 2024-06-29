fn main() {
    // Listing 13-10: Creating an iterator
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // Listing 13-11: Using an iterator in a for loop
    for val in v1_iter {
        println!("Got: {val}");
    }

    let v1: Vec<i32> = vec![1 ,2 , 3];
    
    // Listing 13-14: Calling the iterator adaptor map to 
    // create a new iterator
    // Listing 13-15: Calling the map method to create a
    // new iterator and then calling the collect method to
    // consume the new iterator and create a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
