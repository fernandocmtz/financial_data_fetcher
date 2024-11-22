use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

// Struct definitions
#[derive(Debug, Deserialize)]
struct Bitcoin {
    price_usd: f64,
}

#[derive(Debug, Deserialize)]
struct Ethereum {
    price_usd: f64,
}

#[derive(Debug, Deserialize)]
struct SP500 {
    price_usd: f64,
}

// Trait for standardized pricing operations
trait Pricing {
    fn fetch_price(&mut self);
    fn save_to_file(&self);
}

// Implement Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) {
        if let Some(price) = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()
            .ok()
            .and_then(|r| r.into_string().ok())
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|json| json["bpi"]["USD"]["rate_float"].as_f64())
        {
            self.price_usd = price;
            println!("Fetched Bitcoin price: ${}", self.price_usd);
        } else {
            println!("Failed to fetch Bitcoin price.");
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create("bitcoin.txt").unwrap();
        writeln!(file, "Bitcoin Price: ${}", self.price_usd).unwrap();
    }
}

// Implement Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&mut self) {
        if let Some(price) = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .ok()
            .and_then(|r| r.into_string().ok())
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|json| json["ethereum"]["usd"].as_f64())
        {
            self.price_usd = price;
            println!("Fetched Ethereum price: ${}", self.price_usd);
        } else {
            println!("Failed to fetch Ethereum price.");
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create("ethereum.txt").unwrap();
        writeln!(file, "Ethereum Price: ${}", self.price_usd).unwrap();
    }
}

// Implement Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&mut self) {
        // Placeholder for S&P 500 index; replace with a real API if available
        self.price_usd = 4500.0; // Example static value
        println!("Fetched S&P 500 price: ${}", self.price_usd);
    }

    fn save_to_file(&self) {
        let mut file = File::create("sp500.txt").unwrap();
        writeln!(file, "S&P 500 Price: ${}", self.price_usd).unwrap();
    }
}

// Main function
fn main() {
    let mut bitcoin = Bitcoin { price_usd: 0.0 };
    let mut ethereum = Ethereum { price_usd: 0.0 };
    let mut sp500 = SP500 { price_usd: 0.0 };

    let mut assets: Vec<&mut dyn Pricing> = vec![&mut bitcoin, &mut ethereum, &mut sp500];

    loop {
        for asset in &mut assets {
            asset.fetch_price();
            asset.save_to_file();
        }
        println!("Data saved. Waiting for the next cycle...");
        thread::sleep(Duration::from_secs(10));
    }
}
