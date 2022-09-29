use reqwest;
use scraper::{ Html, Selector };

fn main() {
    use std::time::Instant;

    let now = Instant::now();

    let url = "https://en.wikipedia.org/wiki/Kurt_G%C3%B6del";

    let text_response = reqwest::blocking::get(url) 
        .unwrap()
        .text()
        .unwrap();

    let html = Html::parse_document(text_response.as_str());
    let anchor_selector = Selector::parse("a").unwrap();

    let hrefs = html
        .select(&anchor_selector)
        .map(|a| {
            a.value().attr("href")
        })
        .filter(|href| {
            match href {
                Some(_) => true,
                None => false,
            }
        })
        .collect::<Vec<Option<&str>>>();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    for h in hrefs {
        println!("{:?}", h);
    }
}
