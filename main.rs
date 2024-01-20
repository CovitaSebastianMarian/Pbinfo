use std::io::{self};

use reqwest;
use select::document::Document;
use select::node::Node;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut search_word: String = String::new();
    println!("Introdu cuvatul pe care vrei sa il cauti pe pbinfo:");
    io::stdin().read_line(&mut search_word).unwrap();
    search_word = search_word.trim().to_string();

    println!("[                                                                                                     ]");
    let mut found_links: Vec<(String, String)> = Vec::new();

    for i in 0..=100 {
        let url = format!("https://www.pbinfo.ro/solutii?start={}", i * 50).to_string();
        let response = reqwest::get(url.clone()).await?;

        let body = response.text().await?;
        let document = Document::from(body.as_str());

        let mut found = false;
        for node in document.find(|node: &Node| node.text().contains(search_word.as_str())) {
            let node_text = node.text();
            if let Some(position) = node_text.find(search_word.as_str()) {
                let following_words = &node_text[position..]
                    .split_whitespace()
                    .take(9)
                    .collect::<Vec<&str>>()
                    .join(" ");

                found = true;
                if following_words.len() > search_word.len() + 1 {
                    let link_info = (url.clone(), following_words.clone());
                    if !found_links.contains(&link_info) {
                        found_links.push((url.clone(), following_words.clone()));
                    }
                }
            }
        }
        
        let ch = if found { "$" } else { "_" };
        print!("\x1B[1A");
        println!("{}{}", format!("\x1B[{}C", i + 1).to_string(), ch);
    }
    if found_links.len() != 0 {
        for i in found_links {
            println!("Link: {}, info: {}", i.0, i.1);
        }
    } else {
        println!("Cuvantul {} nu a fost gasit!", search_word);
    }

    Ok(())
}
