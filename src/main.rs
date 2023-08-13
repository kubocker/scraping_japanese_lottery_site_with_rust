
mod scraping;
use scraping::Scraping;


fn main() {

    let num3_lottery = scraping::Num3Lottery {};
    let monthly_result = num3_lottery.get_monthly_result("202308");
    println!("Monthly Result: {:?}", monthly_result);

    let daily_result = num3_lottery.get_daily_result("20230801");
    println!("Daily Result: {:?}", daily_result);

}
