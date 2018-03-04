# Dependencies
## [Rust language](https://www.rust-lang.org/install.html)
Rust is a systems programming language compatible with Linux, OS X, and Windows ([ish](http://www.jonathanturner.org/2017/03/rust-in-windows.html))
## [Crates](https://doc.rust-lang.org/book/first-edition/crates-and-modules.html)
Packages in Rust are known as "crates." The crates used in this repository are described in the [`Cargo.toml`](Cargo.toml) file
### [csv](https://docs.rs/csv)
Used for reading .csv files
### [futures](https://docs.rs/futures)
Used for allowing futures and streams functionality
### [hyper](https://docs.rs/hyper)
Used for sending HTTP requests
### [hyper-rustls](https://docs.rs/hyper-rustls)
Used for enabling HTTPS with requests
### [tokio-core](https://docs.rs/tokio-core)
Used for allowing I/O and event loop abstraction functionality
## [Alpha Vantage API](https://www.alphavantage.co/documentation)
The Alpha Vantage API provides historical daily equity prices adjusted for dividends and splits based on equity symbols

# Usage
## Inserting API key
Get an Alpha Vantage API key [here](https://www.alphavantage.co/support/#api-key). As of March 2018, obtaining an API key is free. After getting the key, replace "INSERT API KEY HERE" in [`main.rs`](src/main.rs) under the `src` directory of this repository with your own API key, keeping the quotation marks intact
## Specifying current portfolio equities
The portfolio should be specified in `symbols.csv` in the repository root directory. An example `symbols.csv` is available in the `example` directory of this repository. The format for `symbols.csv` is as follows:
1. The first row should be the two column titles (e.g. symbol,num_shares in the example `symbols.csv`)
2. Subsequent rows should be the symbol first and the number of shares second (e.g. BABA,5 in the example `symbols.csv` specifies 5 shares of the equity with symbol "BABA" [Alibaba Group Holding Ltd stock])
## Specifying proposed equities
The proposed equities should be specified in `proposed.txt` in the repository root directory. An example `proposed.txt` is available in the `example` directory of this repository. The format for `proposed.txt` is as follows:
1. The symbol of the proposed equity (e.g. MMM in the example `proposed.txt` represents shares of the 3M Company)

# Output
The program outputs for each of the proposed equities the correlation between the daily prices of that equity and the daily total values of the specified portfolio. The number of days with which the correlation is calculated is the minimum of the following:
1. The number of days of data available for the proposed equity on the Alpha Vantage API
2. The number of days of data available for all equities in the specified current portfolio on the Alpha Vantage API

Note that limiting the calculation of the correlation to a specific range of dates is not currently supported
