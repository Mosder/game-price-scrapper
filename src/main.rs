mod scrapper;
use std::error::Error;
use scrapper::Genre;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game_map = scrapper::scrape(
        10.0,
        100.0,
        25,
        Genre::Action
    ).await;
    println!("{:?}", game_map);
    Ok(())
}