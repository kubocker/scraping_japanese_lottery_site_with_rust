extern crate reqwest;
extern crate scraper;
extern crate html5ever;
extern crate tokio;
extern crate html2text;

use scraper::{Html, Selector};
use html2text::from_read;
use std::error::Error;
use html_escape::decode_html_entities;
use std::io::Cursor;

// 返り値の型を仮定して定義
type ReturnType = Vec<Vec<Vec<String>>>;
type ErrorType = Box<dyn Error>;

#[tokio::main]
pub async fn get_monthly_result(input_type: &str, input_monthly: &str) -> Result<ReturnType, ErrorType> {
    let url = format!(
        "https://takarakuji.rakuten.co.jp/backnumber/{}/{}",
        input_type,
        input_monthly
    );

    // HTTPリクエストを送信してHTMLを取得
    let html = reqwest::get(url).await?.text().await?;

    // HTMLをパースしてスクレイピング
    let fragment = Html::parse_document(&html);
    let selector = Selector::parse("table").unwrap();

    // スクレイピング処理
    let mut data = Vec::new();
    for table in fragment.select(&selector) {
        // テーブルの各行を処理

        let mut d2 : Vec<Vec<String>>= Vec::new();
        for row in table.select(&Selector::parse("tr").unwrap()) {
            let mut row_data = Vec::new();

            for cell in row.select(&Selector::parse("th").unwrap()) {
                let text = remove_html_tags(&cell.inner_html());
                let trimmed_text = text.trim_end_matches('\n').to_string();
                if trimmed_text.contains("第") {
                    row_data.push(trimmed_text);
                }
            }

            for cell in row.select(&Selector::parse("td").unwrap()) {
                let text = remove_html_tags(&cell.inner_html());
                let trimmed_text = text.trim_end_matches('\n').to_string();
                row_data.push(trimmed_text);
            }
            d2.push(row_data);
        }
        data.push(d2);
    }
    Ok(data)
    // スクレイピングデータを返す
}

fn remove_html_tags(input: &str) -> String {
    let decoded = decode_html_entities(input);
    let cursor = Cursor::new(decoded.as_bytes());
    let text = from_read(cursor, 80); // html2textを使用してHTMLタグを除去
    text
}


// 金額の解析関数
pub fn parse_price(price_str: &str) -> f64 {
    let cleaned_str = price_str.replace("円", "").replace(",", "");
    cleaned_str.parse().unwrap_or(0.0)
}