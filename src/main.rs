use serde::{Deserialize, Serialize};
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
    // parse the JSON file
    let products: Products =
        serde_json::from_str(file_contents.as_str()).expect("Failed parsing JSON");
    println!("{:?}", products);

    // get input
    // search in products
    let result: Option<&Product> = products
        .products
        .iter()
        .find(|product| product.name == "Widget");

    match result {
        Some(product) => println!("{:?}", product),
        None => println!("Sorry, that product was not found in our inventory."),
    }
    // if not found, Output "Sorry, that product was not found in our inventory."
    // if found, show price, and quantity on hand.
}
