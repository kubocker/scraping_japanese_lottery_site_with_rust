

pub trait Scraping<T> {
    fn get_monthly_result(&self, month: &str) -> Vec<T>;
    // fn get_daily_result(&self, date: &str) -> T;
}
