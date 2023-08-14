# Scraping Japanese Lottery Site

this library is to get lottery monthly result `loto6`, `numbers3` and `numbers4`.
scraping japanese lottery site from `https://takarakuji.rakuten.co.jp/backnumber/`.



### how to use

- input string at `get_monthly_result` function
- input `201605`, if you want to  get result of `2016/05`

ex.
```rust
extern crate scraping_japanese_lottery_site;
use crate::scraping_japanese_lottery_site::scraping::Scraping;
use scraping_japanese_lottery_site::scraping::{num3_lottery, num4_lottery};


fn main() {
  let num3 = num3_lottery::Num3Lottery {};
  let result = num3.get_monthly_result("202308");

  let num4 = num4_lottery::Num4Lottery {};
  let result = num4.get_monthly_result("202308");
}
```