use regex::Regex;
use reqwest::Client;
use tokio::net::{TcpStream};

#[derive(Clone, Copy)]
pub struct Scraper {}

impl Scraper {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_url(&self, client: Client, url: String) -> String {
        let var = client.get(url)
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0")
        .header("connection", "keep-alive")
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("accept-language", "en-GB,en-US;q=0.9,en;q=0.8")
        .header("sec-ch-ua", r#""Chromium";v="102", "Not A;Brand";v="99""#)
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", r#""Linux""#)
        .header("sec-fetch-dest", "document")
        .header("sec-fetch-mode", "navigate")
        .header("sec-fetch-site", "none")
        .header("sec-fetch-user", "?1")
        .header("upgrade-insecure-requests", "1")
        .send().await.unwrap();

        var.text().await.unwrap()
    }

    pub async fn scrape(&self, text: String) -> Vec<String> {
        let mut ips = vec![];
        let ip_regex = Regex::new(r#"\d{1,3}(?:\.\d{1,3}){3}(?::\d{1,5})?"#).unwrap();
        for ip in ip_regex.find_iter(&text) {
            ips.push(ip.as_str().to_string());
        }
        ips
    }

    pub async fn check_proxy(&self, addr: String, port: String) -> Option<String> {
        let other_addr = addr.clone();
        match tokio::time::timeout(tokio::time::Duration::from_millis(500), TcpStream::connect((addr, port.parse::<u16>().unwrap()))).await {
            Ok(x) => match x {
                Ok(_) => Some(format!("{}:{}", other_addr, port)),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
