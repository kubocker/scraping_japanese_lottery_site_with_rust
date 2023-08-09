use scraper::Html;
use scraper::Selector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let result = get_reqwest().await?;
    try_parse_html(&result);
    Ok(())
}

async fn get_reqwest() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://takarakuji.rakuten.co.jp/backnumber/loto6/202307").await?.text().await?;
    Ok(body)
}

fn try_parse_html(html: &str) {
    let document = Html::parse_document(html);
    let selector_str = "th";
    let selector = Selector::parse(selector_str).unwrap();

    for element in document.select(&selector) {
        if let Some(unicode) = element.text().next() {
            println!("{}", unicode);
        }
    }
}