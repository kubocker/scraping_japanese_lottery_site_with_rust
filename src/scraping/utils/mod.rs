extern crate reqwest;
extern crate scraper;
extern crate html5ever;
extern crate tokio;

use scraper::{Html, Selector};
use html5ever::serialize::{serialize, SerializeOpts};
use std::error::Error;
use std::result::Result;

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
                row_data.push(text);
            }
            data.push(row_data);
        }
    }

    println!("{:?}", data);

    println!("HTMLの取得とスクレイピングが完了しました。");
    println!("End");

    // スクレイピングデータを返す
    Ok(data)
}

fn remove_html_tags(input: &str) -> String {
    let mut output = Vec::new();
    let _ = serialize(
        &mut output,
        &scraper::Html::parse_fragment(input),
        SerializeOpts::default(), // SerializeOptsのインスタンスを作成するときに引数なしのdefault()を呼び出す
    );
    String::from_utf8_lossy(&output).to_string()
}