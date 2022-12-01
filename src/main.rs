mod scrapers;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use scrapers::Scraper;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::main;
#[main]
async fn main() {
    http_scrape().await;
    socks_scrape().await;
}
async fn http_scrape() {
    let http_urls = [
        "https://spys.me/proxy.txt",
        "https://api.proxyscrape.com/?request=getproxies&proxytype=http&timeout=1000&country=All",
        "https://www.proxy-list.download/api/v1/get?type=http&anon=elite",
        "https://www.proxy-list.download/api/v1/get?type=http&anon=transparent",
        "https://www.proxy-list.download/api/v1/get?type=http&anon=anonymous",
    ];
    let http_urls = http_urls.map(|x| x.to_string());


    let scraper = Scraper::new();
    let client = reqwest::Client::new();

    let mut texts = vec![];
    let mut futs = FuturesUnordered::new();
    let mut http_urls = http_urls.iter().peekable();
    while let Some(url) = http_urls.next() {
        futs.push(scraper.get_url(client.clone(), url.to_string()));
        if http_urls.peek().is_none() { break }
    }
    while let Some(text) = futs.next().await {
        texts.push(text);
    }

    let mut https = OpenOptions::new()
        .create(true)
        .append(true)
        .open("http.txt")
        .await
        .unwrap();



    let mut futs = FuturesUnordered::new();
    let mut texts = texts.iter().peekable();
    while let Some(text) = texts.next() {
        futs.push(scraper.scrape(text.to_string()));
        if texts.peek().is_none() { break }
    }
    let mut stuff = vec![];
    while let Some(mut ips) = futs.next().await {
        stuff.append(&mut ips)
    }



    let mut futs = FuturesUnordered::new();
    let mut valid_proxies : Vec<String> = vec![];
    let mut stuff = stuff.iter().peekable();
    while let Some(proxy) = stuff.next() {
        let items: Vec<&str> = proxy.split(':').collect();
        futs.push(scraper.check_proxy(items[0].to_string(), items[1].to_string()));
        while let Some(valid) = futs.next().await {
            if let Some(valid) = valid {
                println!("Valid: {valid}");
                valid_proxies.push(valid);
            }
        }
        if stuff.peek().is_none() { break }
    }

    for proxy in valid_proxies {
        https.write_all(format!("{proxy}\n").as_bytes()).await.unwrap();
    }
}

async fn socks_scrape() {
    let scraper = Scraper::new();
    let client = reqwest::Client::new();

    let socks_urls = [
        "https://spys.me/socks.txt",
        "https://api.proxyscrape.com/?request=getproxies&proxytype=socks&timeout=1000&country=All",
        "https://www.proxy-list.download/api/v1/get?type=socks4&anon=elite",
        "https://www.proxy-list.download/api/v1/get?type=socks4&anon=transparent",
        "https://www.proxy-list.download/api/v1/get?type=socks4&anon=anonymous",
        "https://www.proxy-list.download/api/v1/get?type=socks5&anon=elite",
        "https://www.proxy-list.download/api/v1/get?type=socks5&anon=transparent",
        "https://www.proxy-list.download/api/v1/get?type=socks5&anon=anonymous",
    ];

    let socks_urls = socks_urls.map(|x| x.to_string());

    let mut socks = OpenOptions::new()
        .create(true)
        .append(true)
        .open("socks.txt")
        .await
        .unwrap();

    let mut texts = vec![];
    let mut futs = FuturesUnordered::new();
    let mut socks_urls = socks_urls.iter().peekable();

    while let Some(url) = socks_urls.next() {
        futs.push(scraper.get_url(client.clone(), url.to_string()));
        if socks_urls.peek().is_none() { break }
    }
    while let Some(text) = futs.next().await {
        texts.push(text);
    }

    let mut futs = FuturesUnordered::new();
    let mut texts = texts.iter().peekable();
    while let Some(text) = texts.next() {
        futs.push(scraper.scrape(text.to_string()));
        if texts.peek().is_none() { break }
    }
    let mut stuff = vec![];
    while let Some(mut ips) = futs.next().await {
        stuff.append(&mut ips)
    }

    let mut futs = FuturesUnordered::new();
    let mut valid_proxies : Vec<String> = vec![];
    let mut stuff = stuff.iter().peekable();
    while let Some(proxy) = stuff.next() {
        let items: Vec<&str> = proxy.split(':').collect();
        futs.push(scraper.check_proxy(items[0].to_string(), items[1].to_string()));
        while let Some(valid) = futs.next().await {
            if let Some(valid) = valid {
                println!("Valid: {valid}");
                valid_proxies.push(valid);
            }
        }
        if stuff.peek().is_none() { break }
    }

    for proxy in valid_proxies {
        socks.write_all(format!("{proxy}\n").as_bytes()).await.unwrap();
    }

}
