
use crate::scraping::traits::Scraping;
use crate::scraping::types::Numbers3;
use crate::scraping::utils::get_monthly_result;


#[derive(Debug)]
pub struct Num4Lottery;



impl Scraping<Numbers4> for Num4Lottery {
    fn get_monthly_result(&self, month: &str) -> Vec<Numbers4> {
        match get_monthly_result("numbers3", month) {
            Ok(data) => {
                // データの取り出し
                println!("{:?}", data);
                // for v in data.into_iter() {
                //     println!("{}", v):
                // }
            }
            Err(error) => {
                // エラーハンドリング
                eprintln!("Error: {}", error);
            }
        }
        vec![
            Numbers4 {
                no: "1".to_string(),
                date: month.to_string(),
            },
            Numbers4 {
                no: "2".to_string(),
                date: month.to_string(),
            },
            Numbers4 {
                no: "3".to_string(),
                date: month.to_string(),
            },
        ]
    }

    fn get_daily_result(&self, date: &str) -> Numbers4 {
        Numbers4 {
            no: "1".to_string(),
            date: date.to_string(),
            // figure: Default::default(),
            // result: Default::default(),
        }
    }
}
