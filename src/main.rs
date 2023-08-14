mod scraping;
use scraping::{Num3Lottery, Num4Lottery};

use crate::scraping::Scraping;


fn main() {

    let num3_lottery = Num3Lottery {};
    let monthly_result1 = num3_lottery.get_monthly_result("202308");
    println!("Monthly Result: {:?}", monthly_result1);

    let num4_lottery = Num4Lottery {};
    let monthly_result2 = num4_lottery.get_monthly_result("202308");
    println!("Monthly Result: {:?}", monthly_result2);

}
