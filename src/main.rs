
extern crate reqwest;
extern crate scraper;
extern crate html5ever;
extern crate tokio;

use scraper::{Html, Selector};
use html5ever::serialize::{serialize, SerializeOpts};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::result::Result;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://takarakuji.rakuten.co.jp/backnumber/loto6/202307";
    let csv_file = format!("202307.csv");

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

    // println!("{:?}", data);

    // CSVファイルにデータを書き込む
    let mut file = File::create(csv_file)?;
    for row in data {
        let row_str = row.join(",");
        println!("{:?}", row_str);
        writeln!(file, "{}", row_str)?;
    }

    println!("HTMLの取得とスクレイピングが完了しました。");
    println!("End");
    Ok(())
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
