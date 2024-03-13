use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::io;

fn main() {
    let urls = ["https://news.mit.edu/topic/artificial-intelligence2", 
    "https://tr.euronews.com/tag/yapay-zeka", "https://techcrunch.com/category/artificial-intelligence/", "https://yapaymag.com/"];
    loop {
        print!("----------------------------------------------------------------------------------------------- \n");
        println!("Hangi kaynaktan haber okumak istersiniz? (1-MIT, 2-Euronews, 3-TechCrunch 4-YapaMag q-Çıkış)");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Satır okunurken hata oluştu");
        print!("----------------------------------------------------------------------------------------------- \n");
        
        let result = match input1.trim() {
            "1" => urls[0], 
            "2" => urls[1],
            "3" => urls[2],
            "4" => urls[3],
            "q" => break,
            _ => {
                println!("Geçersiz seçim.");
                return;
            },
        };
        let response = get(result).expect("URL alınırken hata oluştu");
        let body = response.text().expect("Yanıt gövdesi okunurken hata oluştu");
        let document = Html::parse_document(&body);

        let selector_mit = Selector::parse("h3 a").unwrap();
        let selector_euronews = Selector::parse("h3 a").unwrap();
        let selector_techcrunch = Selector::parse("h2 a").unwrap();
        let selector_yapaymag = Selector::parse("h2 a").unwrap();
        let mut titles = Vec::new();
        let selector = match input1.trim() {
            "1" => &selector_mit,
            "2" => &selector_euronews,
            "3" => &selector_techcrunch,
            "4" => &selector_yapaymag,
            "q" => break,
            _ => {
                println!("Geçersiz seçim.");
                return;
            },
        };
        for (index, node) in document.select(&selector).enumerate() {
            let title = node.text().collect::<Vec<_>>().join(" ");
            println!("{}: {}", index, title);
            titles.push((title, node.value().attr("href").unwrap().to_string()));
        }
        print!("----------------------------------------------------------------------------------------------- \n");
        println!("Okumak istediğiniz Haberin numarasını giriniz:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Satır okunurken hata oluştu");
        print!("----------------------------------------------------------------------------------------------- \n");
        
        let choice: usize = input.trim().parse().expect("Lütfen bir numara yazın!");
        let heads = ["https://news.mit.edu/", "https://tr.euronews.com/", "https://techcrunch.com","https://yapaymag.com"];

        if let Some((_, article_url)) = titles.get(choice) {
            let full_url = if article_url.starts_with("http") {
                article_url.to_string()
            } else {
                match input1.trim() {
                    "1" => format!("{}{}", heads[0], article_url.trim_start_matches('/')),
                    "2" => format!("{}{}", heads[1], article_url.trim_start_matches('/')),
                    "3" => format!("{}{}", heads[2], article_url.trim_start_matches('/')),
                    "4" => format!("{}{}", heads[3], article_url.trim_start_matches('/')),
                    "q" => break,
                    _ => {
                        println!("Geçersiz seçim.");
                        return;
                    },
                }
            };
            let response = get(&full_url).expect("Makale URL'si alınırken hata oluştu");
            let body = response.text().expect("Makale gövdesi okunurken hata oluştu");
            let document = Html::parse_document(&body);

            let selector = Selector::parse("p").unwrap();
            for paragraph in document.select(&selector) {
                let text = paragraph.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !text.is_empty() {
                    println!("{}", text);
                }
            }
        }
        if input1.trim() == "q" {
            break;
        }
    }
    
}