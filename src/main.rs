use reqwest;
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::task;

async fn ping_url(url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let response = reqwest::get(url).await?;
    Ok(response.status())
}

async fn ping_urls(urls: Vec<&str>) {
    let mut handles = Vec::new();

    for url in urls {
        let url = url.to_string();
        let handle = task::spawn(async move {
            match ping_url(&url).await {
                Ok(status) => println!("URL: {}, Status: {}", url, status),
                Err(e) => println!("URL: {}, Error: {}", url, e),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
}

async fn ping_urls_lim(urls: Vec<&str>, concurrency_limit: usize) {
    let semaphore = Arc::new(Semaphore::new(concurrency_limit)); // Limit to concurrency_limit concurrent requests
    let mut handles = Vec::new();

    for url in urls {
        let url = url.to_string();
        let semaphore = semaphore.clone();
        let handle = task::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            match ping_url(&url).await {
                Ok(status) => println!("URL: {}, Status: {}", url, status),
                Err(e) => println!("URL: {}, Error: {}", url, e),
            }
            drop(_permit);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.example.com",
        "https://www.github.com",
        "https://www.stackoverflow.com",
        "https://www.reddit.com",
        "https://www.twitter.com",
        "https://www.youtube.com",
        "https://www.wikipedia.org",
        "https://www.amazon.com",
        "https://nonexistent.url",
    ];

    println!("Pinging URLs...");
    //ping_urls(urls).await;
    ping_urls_lim(urls, 3).await; // Limit to 3 concurrent requests
    println!("Done!");
}

// TODO
// - Add command-line argument parsing to specify URLs and concurrency limit
// - Implement error handling and retries for failed requests
// - Add logging instead of printing to console
// - Input from a file or other sources
// - Save results to a file
// - Track response times and other metrics
