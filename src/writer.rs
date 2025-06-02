use std::fs;
use std::error::Error;
use std::collections::HashMap;
use crate::scrapper::GameData;
use chrono::prelude::{DateTime, Utc};
use std::time::SystemTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GameRecord {
    name: String,
    g2a_price: f32,
    g2a_sale: i32,
    g2a_link: String,
    kinguin_price: f32,
    kinguin_sale: i32,
    kinguin_link: String,
    cdkeys_price: f32,
    cdkeys_sale: i32,
    cdkeys_link: String,
}

impl GameRecord {
    fn from(name: &str, data: GameData) -> GameRecord {
        GameRecord {
            name: name.to_string(),
            g2a_price: if let Some(ref x) = data.g2a {x.price} else {0.0},
            g2a_sale: if let Some(ref x) = data.g2a {x.sale} else {0},
            g2a_link: if let Some(x) = data.g2a {x.link} else {String::from("-")},
            kinguin_price: if let Some(ref x) = data.kinguin {x.price} else {0.0},
            kinguin_sale: if let Some(ref x) = data.kinguin {x.sale} else {0},
            kinguin_link: if let Some(x) = data.kinguin {x.link} else {String::from("-")},
            cdkeys_price: if let Some(ref x) = data.cdkeys {x.price} else {0.0},
            cdkeys_sale: if let Some(ref x) = data.cdkeys {x.sale} else {0},
            cdkeys_link: if let Some(x) = data.cdkeys {x.link} else {String::from("-")}
        }
    }
}

fn mk_csv_dir(dirname: &str) -> Result<(), Box<dyn Error>> {
    if let Ok(false) = fs::exists(dirname) {
        fs::create_dir(dirname)?;
    }
    Ok(())
}

pub fn write_to_csv(dirname: &str, game_map: HashMap<String, GameData>) {
    // file name
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.clone().into();
    let file_path = format!("{}/scrape-{}.csv", dirname, datetime.format("%Y%m%d-%H%M%S"));

    // create a dir if it doesn't exist
    if mk_csv_dir(dirname).is_err() {
        println!("Couldn't create/access directory {}", dirname);
        return
    }

    // iterate over and save every game record
    if let Ok(mut wrtr) = csv::Writer::from_path(file_path) {
        for (name, data) in game_map {
            let game_record = GameRecord::from(&name, data);
            if let Err(_) = wrtr.serialize(game_record) {
                println!("Couldn't serialize game: {}", name);
            }
        }
    }
    else {println!("Couldn't initialize writer");}
}