use reqwest::header::{self, HeaderMap};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;
use csv::Writer; // Make sure this is imported

async fn fetch_url(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0"));

    let response = client.get(url).headers(headers).send().await?;
    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(format!("Failed to fetch URL: {}", response.status()).into())
    }
}

fn scrape_website(html: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("h2").unwrap(); // Adjust this to your needs

    Ok(document
        .select(&selector)
        .map(|element| element.inner_html())
        .collect())
}

fn save_to_csv(data: &[String], filename: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;

    let mut wtr = Writer::from_writer(file);
    for item in data {
        wtr.write_record(&[item])?;
    }
    wtr.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://example.com", // Replace with your target URLs
        // Add more URLs as needed
    ];

    let client = Client::new();
    let mut all_data = Vec::new();

    for url in urls {
        match fetch_url(&client, url).await {
            Ok(html) => {
                match scrape_website(&html) {
                    Ok(data) => {
                        all_data.extend(data.clone()); // Clone to avoid moving
                        println!("Successfully scraped {} elements from {}", data.len(), url);
                    }
                    Err(e) => eprintln!("Error scraping website: {}", e),
                }
            }
            Err(e) => eprintln!("Error fetching URL: {}", e),
        }

        // Rate limiting: wait for 1 second before the next request
        thread::sleep(Duration::from_secs(1));
    }

    if let Err(e) = save_to_csv(&all_data, "data.csv") {
        eprintln!("Error saving to CSV: {}", e);
    } else {
        println!("Data saved to data.csv");
    }
}
