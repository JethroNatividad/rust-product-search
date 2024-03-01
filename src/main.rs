use serde::Deserialize;
use std::fs;
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

fn main() {
    // Read the file "source/products.json"
    let file_contents: String =
        fs::read_to_string("src/products.json").expect("Error reading file");
    println!("File contents:\n{}", file_contents);
    // parse the JSON file
    // get input
    // search in products
    // if not found, Output "Sorry, that product was not found in our inventory."
    // if found, show price, and quantity on hand.
}
