
use crate::scraping::traits::Scraping;
use crate::scraping::types::Loto6;
use crate::scraping::utils::get_monthly_result;


#[derive(Debug)]
pub struct Loto6Lottery;



impl Scraping<Loto6> for Loto6Lottery {
    fn get_monthly_result(&self, month: &str) -> Vec<Loto6> {
        match get_monthly_result("loto6", month) {
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
            Loto6 {
                no: "1".to_string(),
                date: month.to_string(),
            },
            Loto6 {
                no: "2".to_string(),
                date: month.to_string(),
            },
            Loto6 {
                no: "3".to_string(),
                date: month.to_string(),
            },
        ]
    }

    fn get_daily_result(&self, date: &str) -> Loto6 {
        Loto6 {
            no: "1".to_string(),
            date: date.to_string(),
            // figure: Default::default(),
            // result: Default::default(),
        }
    }
}
