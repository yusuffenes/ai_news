use reqwest::blocking::get;
use scraper::{ selector, Html, Selector};
use std::io;


fn main() {
    let urls = ["https://news.mit.edu/topic/artificial-intelligence2", "https://scitechdaily.com/tag/artificial-intelligence/",
    "https://venturebeat.com/category/ai/", "https://techcrunch.com/category/artificial-intelligence/"
    , "https://yapaymag.com/","https://www.artificialintelligence-news.com/"
    ,"https://www.foxbusiness.com/category/artificial-intelligence","https://www.adweek.com/category/artificial-intelligence/"
    ,"https://theconversation.com/europe/topics/artificial-intelligence-ai-90"];
    loop {
        print!("----------------------------------------------------------------------------------------------- \n");
        println!("Hangi kaynaktan haber okumak istersiniz? ");
        println!("1: MIT News");
        println!("2: SciTech Daily");
        println!("3: Venture Beat");
        println!("4: TechCrunch");
        println!("5: Yapay Mag");
        println!("6: Artificial Intelligence News");
        println!("7: Fox Business ");
        println!("8: Adweek AI");
        println!("9: The Conversation");
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
        let selector_scitechdaily = Selector::parse("h3 a").unwrap();
        let selector_venture = Selector::parse("h2 a").unwrap();
        let selector_techcrunch = Selector::parse("h2 a").unwrap();
        let selector_yapaymag = Selector::parse("h2 a").unwrap();
        let selector_artificialintelligence = Selector::parse("h3 a").unwrap();
        let selector_foxbusiness = Selector::parse("h3 a").unwrap();
        let selector_adweek = Selector::parse("h2 a").unwrap();
        let selector_theconversation = Selector::parse("h2 a").unwrap();
        let mut titles = Vec::new();
        let selector = match input1.trim() {
            "1" => &selector_mit,
            "2" => &selector_scitechdaily, 
            "3" => &selector_venture,
            "4" => &selector_techcrunch,
            "5" => &selector_yapaymag,
            "6" => &selector_artificialintelligence,
            "7" => &selector_foxbusiness,
            "8" => &selector_adweek,
            "9" => &selector_theconversation,
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
        let heads = ["https://news.mit.edu/","https://scitechdaily.com/", "https://venturebeat.com/", "https://techcrunch.com","https://yapaymag.com"
        ,"https://www.artificialintelligence-news.com/", "https://www.foxbusiness.com/","https://www.adweek.com/","https://theconversation.com/"];

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

            let selector_mit = Selector::parse(".paragraph--type--content-block-text p").unwrap();
            let selector= Selector::parse("p").unwrap();
            let selector_fox = Selector::parse(".article-body").unwrap();
            let selector_thecon = Selector::parse(".content-body p").unwrap();
            let selector_scidaily = Selector::parse(".entry-content.clearfix > p").unwrap();
            let selector_ainews = Selector::parse("div.cell p").unwrap();
            




            

            let selector = match input1.trim() {
                "1" => &selector_mit,
                "2" => &selector_scidaily,
                "3" => &selector,
                "4" => &selector,
                "5" => &selector,
                "6" => &selector_ainews,
                "7" => &selector_fox,
                "8" => &selector,
                "9" => &selector_thecon,
                "q" => break,
                _ => {
                    println!("Geçersiz seçim.");
                    return;
                },
                
            };
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