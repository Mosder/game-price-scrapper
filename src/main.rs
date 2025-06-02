mod scrapper;
mod writer;

use scrapper::Genre;
use std::io;

const DEFAULT_MIN_PRICE: f32 = 0.0;
const DEFAULT_MAX_PRICE: f32 = 150.0;
const DEFAULT_MIN_SALE: i32 = 30;
const DEFAULT_GENRE_INDEX: u8 = 0;

const CSV_DIR_NAME: &str = "csv";

fn rdln(message: String) -> String {
    let mut buffer = String::new();
    println!("{}", message);
    let _ = io::stdin().read_line(&mut buffer);
    buffer
}

fn get_genre_from_index(index: u8) -> Genre {
    match index {
        1 => Genre::Adventure,
        2 => Genre::Rpg,
        3 => Genre::Strategy,
        4 => Genre::Horror,
        5 => Genre::Puzzle,
        6 => Genre::Casual,
        _ => Genre::Action
    }
}

#[tokio::main]
async fn main() {
    // input messages
    let min_price_msg = format!("Input minimum price (in PLN) (default: {DEFAULT_MIN_PRICE}):");
    let max_price_msg = format!("Input maximum price (in PLN) (default: {DEFAULT_MAX_PRICE}):");
    let min_sale_msg = format!("Input minimum sale (in %) (default: {DEFAULT_MIN_SALE}):");
    let genre_index_msg = format!(
        "Input genre index (default: {DEFAULT_GENRE_INDEX}):{}{}{}{}{}{}{}",
        "\n 0 - Action",
        "\n 1 - Adventure",
        "\n 2 - Rpg",
        "\n 3 - Strategy",
        "\n 4 - Horror",
        "\n 5 - Puzzle",
        "\n 6 - Casual"
    );

    // initialize default values
    let mut min_price: f32 = 0.0;
    let mut max_price: f32 = 150.0;
    let mut min_sale: i32 = 30;
    let mut genre_index: u8 = 0;

    // read values from stdin
    if let Ok(x) = rdln(min_price_msg).trim().parse::<f32>() {
        if x >= 0.0 {
            min_price = x;
        }
    }
    if let Ok(x) = rdln(max_price_msg).trim().parse::<f32>() {
        if x >= 0.0 {
            max_price = x;
        }
    }
    if let Ok(x) = rdln(min_sale_msg).trim().parse::<i32>() {
        if x >= 0 {
            min_sale = x;
        }
    }
    if let Ok(x) = rdln(genre_index_msg).trim().parse::<u8>() {
        if x < 7 {
            genre_index = x;
        }
    }

    // scrape
    let game_map = scrapper::scrape(
        min_price,
        max_price,
        min_sale,
        get_genre_from_index(genre_index)
    ).await;

    // save to csv
    writer::write_to_csv(CSV_DIR_NAME, game_map);
}