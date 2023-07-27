use rand::prelude::*;
use std::error::Error;
use std::time::Instant;
use csv::Writer;

#[allow(dead_code)]
struct StockData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: f64,
}

impl StockData {
    fn from_row(row: &csv::StringRecord) -> StockData {
        StockData {
            date: row[0].to_string(),
            open: row[1].parse().unwrap_or(0.0),
            high: row[2].parse().unwrap_or(0.0),
            low: row[3].parse().unwrap_or(0.0),
            close: row[4].parse().unwrap_or(0.0),
            adj_close: row[5].parse().unwrap_or(0.0),
            volume: row[6].parse().unwrap_or(0.0),
        }
    }
}

fn read_stock_data(file_path: &str) -> Result<Vec<StockData>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut stock_data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        stock_data.push(StockData::from_row(&record));
    }

    Ok(stock_data)
}

fn monte_carlo_simulation(_ticker: &str, initial_price: f64, days: i32, volatility: f64, iterations: i32) -> (f64, Vec<f64>) {
    let stock_data = read_stock_data("./src/data.csv").expect("Error reading data.csv");
    let last_close_price = stock_data.last().map_or(initial_price, |data| data.close);

    let dt = 1.0 / days as f64;
    let drift = (last_close_price - initial_price) / initial_price;
    let mut total_final_price = 0.0;

    let mut rng = rand::thread_rng(); // Initialize random number generator once before the loop

    let mut final_prices = Vec::new();

    for _ in 0..iterations {
        let mut price = initial_price;
        for _ in 0..days {
            let standard_normal = rng.gen::<f64>();
            let random_number = standard_normal * volatility.sqrt() * dt.sqrt();
            price *= 1.0 + (drift * dt + random_number);
        }
        total_final_price += price;
        final_prices.push(price);
    }

    let average_final_price = total_final_price / iterations as f64;
    (average_final_price, final_prices)
}

fn main() {
    let ticker = "NOKIA"; // stock ticker
    let initial_price = 3.97; // current price
    let days = 252; // days to speculate
    let volatility = 0.5; // stock volatility
    let iterations = 1000000; // number of iterations

     // Start the timer before running the simulation
     let start_time = Instant::now();
     let (final_price, final_prices) = monte_carlo_simulation(ticker, initial_price, days, volatility, iterations); 
     // Calculate the elapsed time after the simulation
     let elapsed_time = start_time.elapsed();

    println!(
        "\n{} MC Simulations: {}\nFinal Price: ${} \nInitial price: ${} \nNumber of days: {} \nStock Volatility: {} \n",
        ticker,
        iterations,
        final_price.round() as i64,
        initial_price,
        days,
        volatility
    );
    // Print the elapsed time in seconds and milliseconds
    println!("Elapsed Time: {:.2?} seconds \n", elapsed_time);

    // Save final prices to a CSV file
    save_prices_to_csv("./scripts/output.csv", &final_prices).expect("Error saving prices.csv");
}

//save final price for each iteration
fn save_prices_to_csv(file_path: &str, final_prices: &[f64]) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(file_path)?;

    for price in final_prices {
        writer.write_record(&[price.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}
