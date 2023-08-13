mod utils;
mod types;

use utils::get_monthly_result;
use types::Numbers3;

pub trait Scraping<T> {
    fn get_monthly_result(&self, month: &str) -> Vec<T>;
    fn get_daily_result(&self, date: &str) -> T;
}

#[derive(Debug)]
pub struct Num3Lottery {}

impl Scraping<Numbers3> for Num3Lottery {
    fn get_monthly_result(&self, month: &str) -> Vec<Numbers3> {
        if let Err(err) = get_monthly_result("loto6", month) {
            eprintln!("エラー: {}", err);
        }
        vec![
            Numbers3 {
                no: "1".to_string(),
                date: month.to_string(),
            },
            Numbers3 {
                no: "2".to_string(),
                date: month.to_string(),
            },
            Numbers3 {
                no: "3".to_string(),
                date: month.to_string(),
            },
        ]
    }

    fn get_daily_result(&self, date: &str) -> Numbers3 {
        Numbers3 {
            no: "1".to_string(),
            date: date.to_string(),
            // figure: Default::default(),
            // result: Default::default(),
        }
    }
}