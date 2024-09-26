


# Web Scraper in Rust

A simple web scraping application written in Rust that fetches data from specified URLs and extracts specific HTML elements. This project utilizes the `reqwest` and `scraper` libraries for making HTTP requests and parsing HTML, respectively.

## Features

- Fetches HTML content from specified URLs
- Extracts specified elements (e.g., `<h2>` tags)
- Saves scraped data to a CSV file
- Includes basic error handling
- Implements rate limiting to avoid overwhelming servers

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50 or later)
- Cargo (comes with Rust)

### Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/yourusername/web_scraper.git
   cd web_scraper
   

2. Install the necessary dependencies:

   Add the following lines to your `Cargo.toml` file:

   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["json", "blocking"] }
   scraper = "0.12"
   tokio = { version = "1", features = ["full"] }
   csv = "1.1"
   ```

3. Build the project:

   ```bash
   cargo build
   ```

### Usage

1. Modify the `urls` array in `src/main.rs` to include the URLs you want to scrape.

2. Run the application:

   ```bash
   cargo run
   ```

3. After the program finishes, check the `data.csv` file for the scraped data.

### Example

The current implementation extracts all `<h2>` tags from the specified URLs. You can modify the selector in the `scrape_website` function to target different HTML elements as needed.

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rust](https://www.rust-lang.org/) - The programming language used for this project.
- [reqwest](https://docs.rs/reqwest/) - For making HTTP requests.
- [scraper](https://docs.rs/scraper/) - For parsing HTML content.
- [csv](https://docs.rs/csv/) - For writing CSV files.
```

You can copy everything inside the triple backticks and paste it into your `README.md` file. Let me know if you need anything else!
