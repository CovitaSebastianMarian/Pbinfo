use std::fs;
use std::io::{self, Read, Write};

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
                                
                let text = node_text[position..].split_whitespace().collect::<Vec<&str>>();
                let mut vec: Vec<&str> = Vec::new();
                for i in 0..text.len() {
                    vec.push(text[i]);
                    if text[i] == "Evaluare" {
                        vec.push(text[i + 1]);
                        vec.push(text[i + 2]);
                        break;
                    }
                }
                let following_words = vec.join(" ");
                

                found = true;
                if following_words.len() > search_word.len() + 1 {
                    let link_info = (url.clone(), following_words.clone());
                    if !found_links.contains(&link_info) {
                        found_links.push((url.clone(), following_words.clone()));
                    }
                }
            }
        }
        
        let ch = if found { "#" } else { "_" };
        print!("\x1B[1A");
        println!("{}{}", format!("\x1B[{}C", i + 1).to_string(), ch);
    }
    if found_links.len() != 0 {
        for i in found_links.clone() {
            let log = format!("[Link] {}\n[Details] {}\n\n", i.0, i.1);
            print!("{}", log);
        }
    } else {
        println!("Cuvantul {} nu a fost gasit!", search_word);
    }
    

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    if buffer.trim() == "save" && found_links.len() != 0 {

        let mut open_history = fs::File::open("history.txt").unwrap();
        let mut history_buffer = String::new();
        open_history.read_to_string(&mut history_buffer).unwrap();

        for i in 0..found_links.len() {
            while found_links.len() > i && history_buffer.contains(found_links[i].clone().1.as_str()) {
                found_links.remove(i);
            }
        }
        

        if found_links.len() != 0 {
            let mut file = fs::OpenOptions::new().write(true).append(true).open("history.txt").unwrap();
            for i in found_links {
                //let log = format!("\n[Link] {}\n[Details] {}\n", i.0, i.1);
                let log = format!("\n[Details] {}\n", i.1);
                file.write_all(log.as_bytes()).unwrap();
            }
        }
    }

    Ok(())
}