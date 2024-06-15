fn main() {
    // Listing 8-1: Creating a new, empty vector to hold values 
    //of type i32
    let v: Vec<i32> = Vec::new();

    // Listing 8-2: Creating a new vector containing values
    let v = vec![1, 2 ,3];

    // Listing 8-3: Using the push method to add values to 
    // a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Listing 8-4: Using indexing syntax or the get 
    // method to access an item in a vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // Iterating over the values in the vector
    // Listing 8-7: Printing each element in a vector 
    // by iterating over the elements using a for loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Listing 8-8: Iterating over mutable references to elements 
    // in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Listing 8-9: Defining an enum to store values of 
    // different types in one vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Listing 8-10: Showing where the vector and its 
    // elements are dropped
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here


}
