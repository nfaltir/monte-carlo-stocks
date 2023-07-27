import yfinance as yf
import pandas as pd

def get_stock_data(ticker):
    try:
        # Get historical stock data from Yahoo Finance
        stock_data = yf.download(ticker, period='max')
        return stock_data
    except Exception as e:
        print(f"Error fetching data for {ticker}: {e}")
        return None

def save_to_csv(stock_data, filename):
    try:
        # Save the stock data to a CSV file
        stock_data.to_csv(filename)
        print(f"Data saved to {filename}")
    except Exception as e:
        print(f"Error saving data to CSV: {e}")

def calculate_volatility(file_path):
    # Read the historical stock data from the CSV file
    stock_data = pd.read_csv(file_path)

    # Calculate daily returns
    stock_data['Daily_Return'] = stock_data['Adj Close'].pct_change()

    # Calculate volatility (standard deviation of daily returns)
    volatility = stock_data['Daily_Return'].std()

    return volatility

def main():
    ticker = input("\nEnter stock ticker: ").strip().upper()

    stock_data = get_stock_data(ticker)
    if stock_data is not None:
        save_to_csv(stock_data, "../src/data.csv")

    
    stock_name = ticker
    file_path = "../src/data.csv"  # Replace with the path to your data.csv file

    volatility = calculate_volatility(file_path)

    df = pd.read_csv("../src/data.csv")
    initial_price = df["Adj Close"].iloc[-1]

    print(f"\n{stock_name} Price Volatility: {volatility:.4f}")
    print(f"{stock_name} Initial Price: ${round(initial_price,2)}\n")
        

if __name__ == "__main__":
    main()
