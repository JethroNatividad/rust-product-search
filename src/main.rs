use serde::Deserialize;
use std::fs;
use std::io;
use std::io::Write;
// A program that takes a product name, searches it in a json file, outputs the price and quantity.
// Inputs: product name
// Process: parse json file, search for str
// Outputs: price and quantity if found.

// Create struct for json
#[derive(Debug, Deserialize)]
struct Products {
    products: Vec<Product>,
}

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
    price: f64,
    quantity: i64,
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // Read the file "source/products.json"
    let file_contents: String =
        fs::read_to_string("src/products.json").expect("Error reading file");
    // parse the JSON file
    let products: Products =
        serde_json::from_str(file_contents.as_str()).expect("Failed parsing JSON");

    // get input
    let search_query: String = get_input("What is the product name? ");
    // search in products
    let result: Option<&Product> = products
        .products
        .iter()
        .find(|product| product.name.to_lowercase() == search_query.to_lowercase());

    match result {
        // if found, show price, and quantity on hand.
        Some(product) => println!(
            "Name: {}\nPrice: ${}\nQuantity on hand: {}",
            product.name, product.price, product.quantity
        ),
        // if not found, Output "Sorry, that product was not found in our inventory."
        None => println!("Sorry, that product was not found in our inventory."),
    }
}
