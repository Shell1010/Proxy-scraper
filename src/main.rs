mod scrapers;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use scrapers::Scraper;
use tokio::{main, join};
#[main]
async fn main() {
    join!(http_scrape(), socks_scrape());
}
async fn http_scrape() {
    let http_urls = [
        "https://openproxy.space/list/http",
        "https://spys.me/proxy.txt",
        "https://openproxylist.xyz/http.txt",
        "https://proxyscan.io/download?type=http",
        "https://proxysearcher.sourceforge.net/Proxy%20List.php?type=http",
        "https://proxyspace.pro/http.txt",
        "https://proxyspace.pro/https.txt",
        "https://raw.githubusercontent.com/almroot/proxylist/master/list.txt",
        "https://raw.githubusercontent.com/andigwandi/free-proxy/main/proxy_list.txt",
        "https://raw.githubusercontent.com/aslisk/proxyhttps/main/https.txt",
        "https://raw.githubusercontent.com/B4RC0DE-TM/proxy-list/main/HTTP.txt",
        "https://raw.githubusercontent.com/hanwayTech/free-proxy-list/main/http.txt",
        "https://raw.githubusercontent.com/hanwayTech/free-proxy-list/main/https.txt",
        "https://raw.githubusercontent.com/hendrikbgr/Free-Proxy-Repo/master/proxy_list.txt",
        "https://raw.githubusercontent.com/HyperBeats/proxy-list/main/http.txt",
        "https://raw.githubusercontent.com/jetkai/proxy-list/main/online-proxies/txt/proxies-http.txt",
        "https://raw.githubusercontent.com/jetkai/proxy-list/main/online-proxies/txt/proxies-https.txt",
        "https://raw.githubusercontent.com/mertguvencli/http-proxy-list/main/proxy-list/data.txt",
        "https://raw.githubusercontent.com/mmpx12/proxy-list/master/http.txt",
        "https://raw.githubusercontent.com/mmpx12/proxy-list/master/https.txt",
        "https://raw.githubusercontent.com/ObcbO/getproxy/master/http.txt",
        "https://raw.githubusercontent.com/ObcbO/getproxy/master/https.txt",
        "https://raw.githubusercontent.com/proxy4parsing/proxy-list/main/http.txt",
        "https://raw.githubusercontent.com/proxylist-to/proxy-list/main/http.txt",
        "https://raw.githubusercontent.com/roosterkid/openproxylist/main/HTTPS_RAW.txt",
        "https://raw.githubusercontent.com/rx443/proxy-list/main/online/http.txt",
        "https://raw.githubusercontent.com/rx443/proxy-list/main/online/https.txt",
        "https://raw.githubusercontent.com/saisuiu/uiu/main/free.txt",
        "https://raw.githubusercontent.com/saschazesiger/Free-Proxies/master/proxies/http.txt",
        "https://raw.githubusercontent.com/ShiftyTR/Proxy-List/master/http.txt",
        "https://raw.githubusercontent.com/ShiftyTR/Proxy-List/master/https.txt",
        "https://raw.githubusercontent.com/sunny9577/proxy-scraper/master/proxies.txt",
        "https://raw.githubusercontent.com/TheSpeedX/PROXY-List/master/http.txt",
        "https://raw.githubusercontent.com/yemixzy/proxy-list/main/proxy-list/data.txt",
        "https://raw.githubusercontent.com/Zaeem20/FREE_PROXIES_LIST/master/http.txt",
        "https://raw.githubusercontent.com/Zaeem20/FREE_PROXIES_LIST/master/https.txt",
        "https://raw.githubusercontent.com/zevtyardt/proxy-list/main/http.txt",
        "https://rootjazz.com/proxies/proxies.txt",
        "https://sheesh.rip/http.txt",
        "https://www.freeproxychecker.com/result/http_proxies.txt",
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
    let mut stuff = stuff.iter().peekable();
    while let Some(proxy) = stuff.next() {
        let items: Vec<&str> = proxy.split(':').collect();
        futs.push(scraper.check_proxy(items[0].to_string(), items[1].to_string(), "http.txt"));
        while (futs.next().await).is_some() {}
        if stuff.peek().is_none() { break }
    }


}

async fn socks_scrape() {
    let scraper = Scraper::new();
    let client = reqwest::Client::new();

    let socks_urls = [
        "https://openproxy.space/list/socks4",
        "https://openproxylist.xyz/socks4.txt",
        "https://proxyscan.io/download?type=socks4",
        "https://proxyscan.io/download?type=socks4",
        "https://proxysearcher.sourceforge.net/Proxy%20List.php?type=socks",
        "https://proxyspace.pro/socks4.txt",
        "https://raw.githubusercontent.com/B4RC0DE-TM/proxy-list/main/SOCKS4.txt",
        "https://raw.githubusercontent.com/hanwayTech/free-proxy-list/main/socks4.txt",
        "https://raw.githubusercontent.com/HyperBeats/proxy-list/main/socks4.txt",
        "https://raw.githubusercontent.com/jetkai/proxy-list/main/online-proxies/txt/proxies-socks4.txt",
        "https://raw.githubusercontent.com/mmpx12/proxy-list/master/socks4.txt",
        "https://raw.githubusercontent.com/ObcbO/getproxy/master/socks4.txt",
        "https://raw.githubusercontent.com/proxylist-to/proxy-list/main/socks4.txt",
        "https://raw.githubusercontent.com/roosterkid/openproxylist/main/SOCKS4_RAW.txt",
        "https://raw.githubusercontent.com/saschazesiger/Free-Proxies/master/proxies/socks4.txt",
        "https://raw.githubusercontent.com/ShiftyTR/Proxy-List/master/socks4.txt",
        "https://raw.githubusercontent.com/TheSpeedX/PROXY-List/master/socks4.txt",
        "https://raw.githubusercontent.com/Zaeem20/FREE_PROXIES_LIST/master/socks4.txt",
        "https://raw.githubusercontent.com/zevtyardt/proxy-list/main/socks4.txt",
        "https://www.freeproxychecker.com/result/socks4_proxies.txt",
        "https://spys.me/socks.txt",
        "https://api.proxyscrape.com/?request=getproxies&proxytype=socks&timeout=1000&country=All",
        "https://www.proxy-list.download/api/v1/get?type=socks4&anon=elite",
        "https://www.proxy-list.download/api/v1/get?type=socks4&anon=transparent",
        "https://www.proxy-list.download/api/v1/get?type=socks4&anon=anonymous",
        "https://www.proxy-list.download/api/v1/get?type=socks5&anon=elite",
        "https://www.proxy-list.download/api/v1/get?type=socks5&anon=transparent",
        "https://www.proxy-list.download/api/v1/get?type=socks5&anon=anonymous",
        "https://openproxy.space/list/socks5",
        "https://openproxylist.xyz/socks5.txt",
        "https://proxyscan.io/download?type=socks5",
        "https://proxysearcher.sourceforge.net/Proxy%20List.php?type=socks",
        "https://proxyspace.pro/socks5.txt",
        "https://raw.githubusercontent.com/B4RC0DE-TM/proxy-list/main/SOCKS5.txt",
        "https://raw.githubusercontent.com/hanwayTech/free-proxy-list/main/socks5.txt",
        "https://raw.githubusercontent.com/hookzof/socks5_list/master/proxy.txt",
        "https://raw.githubusercontent.com/HyperBeats/proxy-list/main/socks5.txt",
        "https://raw.githubusercontent.com/jetkai/proxy-list/main/online-proxies/txt/proxies-socks5.txt",
        "https://raw.githubusercontent.com/manuGMG/proxy-365/main/SOCKS5.txt",
        "https://raw.githubusercontent.com/mmpx12/proxy-list/master/socks5.txt",
        "https://raw.githubusercontent.com/ObcbO/getproxy/master/socks5.txt",
        "https://raw.githubusercontent.com/proxylist-to/proxy-list/main/socks5.txt",
        "https://raw.githubusercontent.com/roosterkid/openproxylist/main/SOCKS5_RAW.txt",
        "https://raw.githubusercontent.com/saschazesiger/Free-Proxies/master/proxies/socks5.txt",
        "https://raw.githubusercontent.com/ShiftyTR/Proxy-List/master/socks5.txt",
        "https://raw.githubusercontent.com/TheSpeedX/PROXY-List/master/socks5.txt",
        "https://raw.githubusercontent.com/Zaeem20/FREE_PROXIES_LIST/master/socks5.txt",
        "https://raw.githubusercontent.com/zevtyardt/proxy-list/main/socks5.txt",
        "https://www.freeproxychecker.com/result/socks5_proxies.txt"
    ];

    let socks_urls = socks_urls.map(|x| x.to_string());


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

    let mut stuff = stuff.iter().peekable();
    while let Some(proxy) = stuff.next() {
        let items: Vec<&str> = proxy.split(':').collect();
        futs.push(scraper.check_proxy(items[0].to_string(), items[1].to_string(), "socks.txt"));
        while (futs.next().await).is_some() {}
        if stuff.peek().is_none() { break }
    }



}
