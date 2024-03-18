use reqwest::blocking::get;
use scraper::{selector, Html, Selector};
use std::io;

fn main() {
    let urls = ["https://news.mit.edu/topic/artificial-intelligence2", "https://www.euronews.com/tag/artificial-intelligence",
    "https://tr.euronews.com/tag/yapay-zeka", "https://techcrunch.com/category/artificial-intelligence/"
    , "https://yapaymag.com/","https://www.artificialintelligence-news.com/"
    ,"https://www.foxbusiness.com/category/artificial-intelligence","https://www.adweek.com/category/artificial-intelligence/"
    ,"https://theconversation.com/europe/topics/artificial-intelligence-ai-90","https://abcnews.go.com/alerts/artificial-intelligence"];
    loop {
        print!("----------------------------------------------------------------------------------------------- \n");
        println!("Hangi kaynaktan haber okumak istersiniz? ");
        println!("1: MIT News");
        println!("2: EuronewsEN");
        println!("3: EuronewsTR");
        println!("4: TechCrunch");
        println!("5: Yapay Mag");
        println!("6: Artificial Intelligence News");
        println!("7: Fox Business ");
        println!("8: Adweek AI");
        println!("9: The Conversation");
        println!("10: ABC News");
        println!("q: Çıkış");

        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Satır okunurken hata oluştu");
        print!("----------------------------------------------------------------------------------------------- \n");
        
        let result = match input1.trim() {
            "1" => urls[0], 
            "2" => urls[1],
            "3" => urls[2],
            "4" => urls[3],
            "5" => urls[4],
            "6" => urls[5],
            "7" => urls[6],
            "8" => urls[7],
            "9" => urls[8],
            "10" => urls[9],
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
        let selector_euronews_en = Selector::parse("h3 a").unwrap();
        let selector_euronews_tr = Selector::parse("h3 a").unwrap();
        let selector_techcrunch = Selector::parse("h2 a").unwrap();
        let selector_yapaymag = Selector::parse("h2 a").unwrap();
        let selector_artificialintelligence = Selector::parse("h3 a").unwrap();
        let selector_foxbusiness = Selector::parse("h3 a").unwrap();
        let selector_adweek = Selector::parse("h2 a").unwrap();
        let selector_theconversation = Selector::parse("h2 a").unwrap();
        let selector_abcnews = Selector::parse("h2 a").unwrap();
        let mut titles = Vec::new();
        let selector = match input1.trim() {
            "1" => &selector_mit,
            "2" => &selector_euronews_en, 
            "3" => &selector_euronews_tr,
            "4" => &selector_techcrunch,
            "5" => &selector_yapaymag,
            "6" => &selector_artificialintelligence,
            "7" => &selector_foxbusiness,
            "8" => &selector_adweek,
            "9" => &selector_theconversation,
            "10" => &selector_abcnews,
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
        let heads = ["https://news.mit.edu/","https://www.euronews.com/", "https://tr.euronews.com/", "https://techcrunch.com","https://yapaymag.com"
        ,"https://www.artificialintelligence-news.com/", "https://www.foxbusiness.com/","https://www.adweek.com/","https://theconversation.com/","https://abcnews.go.com/"];

        if let Some((_, article_url)) = titles.get(choice) {
            let full_url = if article_url.starts_with("http") {
                article_url.to_string()
            } else {
                match input1.trim() {
                    "1" => format!("{}{}", heads[0], article_url.trim_start_matches('/')),
                    "2" => format!("{}{}", heads[1], article_url.trim_start_matches('/')),
                    "3" => format!("{}{}", heads[2], article_url.trim_start_matches('/')),
                    "4" => format!("{}{}", heads[3], article_url.trim_start_matches('/')),
                    "5" => format!("{}{}", heads[4], article_url.trim_start_matches('/')),
                    "6" => format!("{}{}", heads[5], article_url.trim_start_matches('/')),
                    "7" => format!("{}{}", heads[6], article_url.trim_start_matches('/')),
                    "8" => format!("{}{}", heads[7], article_url.trim_start_matches('/')),
                    "9" => format!("{}{}", heads[8], article_url.trim_start_matches('/')),
                    "10" => format!("{}{}", heads[9], article_url.trim_start_matches('/')),
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