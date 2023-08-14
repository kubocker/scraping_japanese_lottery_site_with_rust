
use crate::scraping::traits::Scraping;
use crate::scraping::types::{Numbers3, Numbers3Result, Result, Numbers3ResultSet, Figure};
use crate::scraping::utils::{get_monthly_result, parse_price};


#[derive(Debug)]
pub struct Num3Lottery;



impl Scraping<Numbers3> for Num3Lottery {
    fn get_monthly_result(&self, month: &str) -> Vec<Numbers3> {
        let mut data: Vec<Numbers3> = Vec::new();

        match get_monthly_result("numbers3", month) {
            Ok(res) => {

                for v in res.iter() {

                    let no = v[0][0].to_string();
                    let date = v[1][0].to_string();
                    let figures: Figure = Figure {
                        num1: v[2][0].parse().unwrap(),
                        num2: v[2][0].to_string()
                    };
                    let straights = Result {
                        num: v[3][0].to_string(),
                        price: v[3][1].to_string()
                    };
                    let boxes = Result {
                        num: v[4][0].to_string(),
                        price: v[4][1].to_string()
                    };
                    let sets = Numbers3ResultSet {
                        straight: Result {
                            num: v[5][0].to_string(),
                            price: v[5][1].to_string()
                        },
                        boxes: Result {
                            num: v[6][0].to_string(),
                            price: v[6][1].to_string()
                        },
                        mini: Result {
                            num: v[7][0].to_string(),
                            price: v[7][1].to_string()
                        }
                    };


                    data.push(Numbers3 {
                        no,
                        date,
                        figures,
                        results: Numbers3Result {
                            straights,
                            boxes,
                            sets
                        }
                    })
                }

            }
            Err(error) => {
                // エラーハンドリング
                eprintln!("Error: {}", error);
            }
        }
        data
    }
}
