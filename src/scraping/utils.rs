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
type ReturnType = Vec<Vec<String>>;
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
        for row in table.select(&Selector::parse("tr").unwrap()) {
            let mut row_data = Vec::new();
            // 行内の各セルを処理
            for cell in row.select(&Selector::parse("td").unwrap()) {
                let text = remove_html_tags(&cell.inner_html());
                let trimmed_text = text.trim_end_matches('\n').to_string();
                row_data.push(trimmed_text);
            }
            data.push(row_data);
        }
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
