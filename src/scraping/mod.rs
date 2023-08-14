
pub mod num3_lottery;
pub mod num4_lottery;
mod traits;
mod types;
mod utils;

pub use num3_lottery::Num3Lottery;
pub use num4_lottery::Num4Lottery;
pub use traits::Scraping;
pub use types::{Numbers3, Loto6};
pub use utils::get_monthly_result;