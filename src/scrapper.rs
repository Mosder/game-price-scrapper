use std::error::Error;
use thirtyfour::{By, WebDriver, DesiredCapabilities};
use thirtyfour::error::WebDriverError;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration};

const DRIVER_PORT: i32 = 9753;

const PLN_TO_USD: f32 = 0.27;
const PLN_TO_EUR: f32 = 0.23;
const PLN_TO_GBP: f32 = 0.20;

const G2A_PAGES: i8 = 3;
const KINGUIN_PAGES: i8 = 3;
const CDKEYS_PAGES: i8 = 3;

pub enum Genre {
    Action, Adventure, Rpg, Strategy, Horror, Puzzle, Casual
}
use Genre::*;

#[derive(Debug)]
pub struct StoreData {
    pub price: f32,
    pub sale: i32,
    pub link: String
}

#[derive(Debug)]
pub struct GameData {
    pub g2a: Option<StoreData>,
    pub kinguin: Option<StoreData>,
    pub cdkeys: Option<StoreData>
}

impl GameData {
    fn new() -> GameData {
        GameData{g2a: None, kinguin: None, cdkeys: None}
    }
}

fn url_g2a(price_min: f32, price_max: f32, genre: &Genre, page: i8) -> String {
    let genre_string = match genre {
        Action => "action-c2699",
        Adventure => "adventure-c1545",
        Rpg => "rpg-c1550",
        Strategy => "strategy-c1551",
        Horror => "horror-c1543",
        Puzzle => "puzzle-c1542",
        Casual => "casual-c2994"
    };
    format!(
        "https://www.g2a.com/category/{}?page={}&price%5Bmax%5D={}&price%5Bmin%5D={}",
        genre_string,
        page,
        price_max * PLN_TO_USD,
        price_min * PLN_TO_USD
    )
}

fn url_kinguin(price_min: f32, price_max: f32, genre: &Genre, page: i8) -> String {
    let genre_string = match genre {
        Action => "1",
        Adventure => "2",
        Rpg => "4",
        Strategy => "3",
        Horror => "28",
        Puzzle => "20",
        Casual => "19"
    };
    format!(
        "https://www.kinguin.net/steam-games?platforms=2&genres={}&active=1&hideUnavailable=0&type=kinguin&priceFrom={}&priceTo={}&phrase=&page={}&size=50&sort=bestseller.score,DESC",
        genre_string,
        price_min * PLN_TO_EUR * 100.0,
        price_max * PLN_TO_EUR * 100.0,
        page - 1
    )
}

fn url_cdkeys(price_min: f32, price_max: f32, genre: &Genre, page: i8) -> String {
    let genre_string = match genre {
        Action => "Action",
        Adventure => "Adventure",
        Rpg => "RPG",
        Strategy => "Strategy",
        Horror => "Horror",
        Puzzle => "Puzzle",
        Casual => "Casual"
    };
    format!(
        "https://www.cdkeys.com/pc/games?genres={}&p={}&price={}-{}",
        genre_string,
        page,
        price_min * PLN_TO_GBP,
        price_max * PLN_TO_GBP
    )
}

async fn scrape_g2a(
    driver: &WebDriver,
    map: &mut HashMap<String, GameData>,
    price_min: f32,
    price_max: f32,
    min_sale: i32,
    genre: &Genre
) -> Result<(), Box<dyn Error>> {
    for page in 1..=G2A_PAGES {
        // go to proper url
        driver.goto(url_g2a(price_min, price_max, genre, page)).await?;
        thread::sleep(Duration::from_secs(2));
        // loop over all games
        for item in driver.find_all(
            By::Css("section > div > ul > li > div.contents > div > div:nth-child(2) > div")
        ).await? {
            // get sale percentage
            let sale_item = item
                .find(By::Css("div.items-end > div:nth-child(1) > div:nth-child(2) > div > div"))
                .await;
            let sale = if sale_item.is_ok() {
                sale_item?
                .text()
                .await?
                .get(1..3)
                .unwrap()
                .parse::<i32>()?
            } else {0};
            // check if sale is at least minimum
            if sale >= min_sale {
                // get rest of values
                let name = item
                    .find(By::Css("div:nth-child(1) > a > h3"))
                    .await?
                    .text()
                    .await?;
                let link = item
                    .find(By::Css("div:nth-child(1) > a"))
                    .await?
                    .prop("href")
                    .await?
                    .unwrap();
                let price = item
                    .find(By::Css("div.items-end > div:nth-child(1) > div:nth-child(1) > div"))
                    .await?
                    .text()
                    .await?
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse::<f32>()?;
                // create StoreData struct
                let store_data = StoreData{price, sale, link};
                // if entry exists, modify it
                if let Some(game_data) = map.get_mut(&name) {
                    game_data.g2a = Some(store_data);
                }
                // if no entry in map, create it
                else {
                    let mut game_data = GameData::new();
                    game_data.g2a = Some(store_data);
                    map.insert(name, game_data);
                }
            }
        }
    }
    Ok(())
}

async fn scrape_kinguin(
    driver: &WebDriver,
    map: &mut HashMap<String, GameData>,
    price_min: f32,
    price_max: f32,
    min_sale: i32,
    genre: &Genre
) -> Result<(), Box<dyn Error>> {
    for page in 1..=KINGUIN_PAGES {
        // go to proper url
        driver.goto(url_kinguin(price_min, price_max, genre, page)).await?;
        thread::sleep(Duration::from_secs(2));
        // loop over all games
        for item in driver.find_all(
            By::Css("div.row > div:nth-child(2) > div > div:nth-child(2) > div:nth-child(2) > div > div:nth-child(3)")
        ).await? {
            // get sale percentage
            let sale_item = item
                .find(By::Css("div:nth-child(3) > div > a"))
                .await;
            let sale = if sale_item.is_ok() {
                sale_item?
                .text()
                .await?
                .get(1..3)
                .unwrap()
                .split('%')
                .next()
                .unwrap()
                .parse::<i32>()?
            } else {0};
            // check if sale is at least minimum
            if sale >= min_sale {
                // get rest of values
                let name_link = item
                    .find(By::Css("div:nth-child(1) > h3 > a"))
                    .await?;
                let name = name_link.text().await?;
                let link = name_link.prop("href").await?.unwrap();
                let price = item
                    .find(By::Css("span[itemprop='lowPrice']"))
                    .await?
                    .text()
                    .await?
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse::<f32>()?;
                // create StoreData struct
                let store_data = StoreData{price, sale, link};
                // if entry exists, modify it
                if let Some(game_data) = map.get_mut(&name) {
                    game_data.kinguin = Some(store_data);
                }
                // if no entry in map, create it
                else {
                    let mut game_data = GameData::new();
                    game_data.kinguin = Some(store_data);
                    map.insert(name, game_data);
                }
            }
        }
    }
    Ok(())
}

async fn scrape_cdkeys(
    driver: &WebDriver,
    map: &mut HashMap<String, GameData>,
    price_min: f32,
    price_max: f32,
    min_sale: i32,
    genre: &Genre
) -> Result<(), Box<dyn Error>> {
    for page in 1..=CDKEYS_PAGES {
        // go to proper url
        driver.goto(url_cdkeys(price_min, price_max, genre, page)).await?;
        thread::sleep(Duration::from_secs(2));
        // loop over all games
        for item in driver.find_all(By::Css("main div.product-item-info")).await? {
            // get sale percentage
            let sale_item = item
                .find(By::Css("span.product-item-discount"))
                .await;
            let sale = if sale_item.is_ok() {
                sale_item?
                .text()
                .await?
                .get(1..3)
                .unwrap()
                .parse::<i32>()?
            } else {0};
            // check if sale is at least minimum
            if sale >= min_sale {
                // get rest of values
                let name_link = item
                    .find(By::Css("a.product-item-link"))
                    .await?;
                let name = name_link.text().await?;
                let link = name_link.prop("href").await?.unwrap();
                let price = item
                    .find(By::Css("span.price"))
                    .await?
                    .text()
                    .await?
                    .get(3..)
                    .unwrap()
                    .parse::<f32>()?;
                // create StoreData struct
                let store_data = StoreData{price, sale, link};
                // if entry exists, modify it
                if let Some(game_data) = map.get_mut(&name) {
                    game_data.cdkeys = Some(store_data);
                }
                // if no entry in map, create it
                else {
                    let mut game_data = GameData::new();
                    game_data.cdkeys = Some(store_data);
                    map.insert(name, game_data);
                }
            }
        }
    }
    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let capabilities = DesiredCapabilities::chrome();
    let driver = WebDriver::new(format!("http://localhost:{DRIVER_PORT}"), capabilities).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

pub async fn scrape(
    price_min: f32,
    price_max: f32,
    min_sale: i32,
    genre: Genre
) -> HashMap<String, GameData> {
    // create an empty hash map to store scrapped data
    let mut map: HashMap<String, GameData> = HashMap::new();
    // panic when failed to initialize driver
    let driver = initialize_driver().await.unwrap();
    // scrape g2a
    if let Err(err) = scrape_g2a(
        &driver, &mut map, price_min, price_max, min_sale, &genre
    ).await {
        println!("Encountered an error with g2a scrapping:\n{:?}\n", err);
    }
    // scrape kinguin
    if let Err(err) = scrape_kinguin(
        &driver, &mut map, price_min, price_max, min_sale, &genre
    ).await {
        println!("Encountered an error with kinguin scrapping:\n{:?}\n", err);
    }
    // scrape cdkeys
    if let Err(err) = scrape_cdkeys(
        &driver, &mut map, price_min, price_max, min_sale, &genre
    ).await {
        println!("Encountered an error with cdkeys scrapping:\n{:?}\n", err);
    }
    map
}