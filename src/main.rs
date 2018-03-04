extern crate csv;
extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate tokio_core;

use std::cmp::{min};
use std::path::{Path};
use csv::{Reader};
use futures::{Future, Stream};
use hyper::{Chunk, Client};
use hyper_rustls::{HttpsConnector};
use std::fs::{File};
use tokio_core::reactor::Core;
use std::io::{BufRead, BufReader, Write};

macro_rules! deviation_iter {
    ($vec:expr, $len:expr, $avg:expr) =>
        ($vec.iter().take($len).map(|elt| elt - $avg))
}

static API_KEY: &str = "INSERT API KEY HERE";

fn main(
) {
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    let mut symbols = Reader::from_path("symbols.csv")
        .expect("Error in opening symbols.csv");
    let portfolio = calculate_portfolio(&mut symbols, &mut core, &client);
    
    let proposed = File::open("proposed.txt").expect("Error in proposed.txt");
    calculate_correlations(&proposed, &portfolio, &mut core, &client);
}

fn calculate_portfolio(
    symbols: &mut Reader<File>,
    core: &mut Core,
    client: &Client<HttpsConnector>,
) -> Vec<f32> {
    let mut portfolio = Vec::new();

    symbols.records().map(|record| {
        let record = record.unwrap();
        let symbol = record.get(0).expect("Error in symbols.csv formatting");
        let prices = download_maybe_and_read_prices(&symbol[..], core, client);

        let num_shares = record.get(1).expect("Error in symbols.csv formatting")
            .parse::<u32>().expect("Error in symbols.csv record formatting");
        add_to_portfolio(prices, num_shares, &mut portfolio);
    }).last();

    portfolio
}

fn download_maybe_and_read_prices(
    symbol: &str,
    core: &mut Core,
    client: &Client<HttpsConnector>,
) -> Vec<f32> {
    if !Path::new(&format!("{}-prices.csv", symbol)).exists() {
        download_prices(symbol, core, client);
    }
    read_prices(symbol)
}

fn download_prices(
    symbol: &str,
    core: &mut Core,
    client: &Client<HttpsConnector>,
) {
    println!("Downloading {}...", symbol);
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&symbol={}&apikey={}&outputsize=full&datatype=csv",
        symbol,
        API_KEY,
    ).parse().expect("Error in URL");

    let mut csv = File::create(format!("{}-prices.csv", symbol))
        .expect("Error in creating prices .csv");
    let work = client.get(url).and_then(|response| {
        response.body().concat2().map(|body: Chunk| {
            csv.write_all(
                body.to_vec().as_slice()
            ).expect("Error in writing")
        })
    });

    core.run(work).expect("Error in core");
}

fn read_prices(
    symbol: &str,
) -> Vec<f32> {
    Reader::from_path(format!("{}-prices.csv", symbol))
        .expect("Error in reading prices .csv").records().map(|record| {
            record.unwrap().get(5).unwrap().parse::<f32>().unwrap()
        }).collect()
}

fn add_to_portfolio(
    prices: Vec<f32>,
    num_shares: u32,
    portfolio: &mut Vec<f32>
) {
    if portfolio.is_empty() {
        *portfolio = prices.iter().map(|price| {
            price * num_shares as f32
        }).collect();
        return;
    }
    portfolio.truncate(prices.len());

    let mut prices_iter = prices.iter();
    portfolio.iter_mut().map(|portfolio_price| {
        prices_iter.next().map(|price| {
            *portfolio_price += price * num_shares as f32;
        })
    }).last();
}

fn calculate_correlations(
    proposed: &File,
    portfolio: &Vec<f32>,
    core: &mut Core,
    client: &Client<HttpsConnector>,
) {
    BufReader::new(proposed).lines().map(|symbol| {
        let symbol = symbol.unwrap();
        let prices = download_maybe_and_read_prices(&symbol[..], core, client);
        calculate_correlation(&symbol[..], prices, &portfolio);
    }).last();
}

fn calculate_correlation(
    symbol: &str,
    prices: Vec<f32>,
    portfolio: &Vec<f32>,
) {
    let min_len = min(prices.len(), portfolio.len());
    let avg_prices: f32 = prices.iter().take(min_len).sum::<f32>()
        / min_len as f32;
    let avg_portfolio: f32 = portfolio.iter().take(min_len).sum::<f32>()
        / min_len as f32;

    let a = deviation_iter!(prices, min_len, avg_prices);
    let b = deviation_iter!(portfolio, min_len, avg_portfolio);
    let ab_sum: f32 = a.zip(b).map(mult_tuple).sum();

    let a = deviation_iter!(prices, min_len, avg_prices);
    let b = deviation_iter!(prices, min_len, avg_prices);
    let a_squared_sum: f32 = a.zip(b).map(mult_tuple).sum();

    let a = deviation_iter!(portfolio, min_len, avg_portfolio);
    let b = deviation_iter!(portfolio, min_len, avg_portfolio);
    let b_squared_sum: f32 = a.zip(b).map(mult_tuple).sum();

    let mut denom: f32 = a_squared_sum * b_squared_sum;
    denom = denom.sqrt();
    let correlation = ab_sum / denom;

    println!(
        "{} correlation based on {} days of data: {:.*}",
        symbol,
        min_len,
        2, correlation
    );
}

fn mult_tuple(
    tuple: (f32, f32),
) -> f32 {
    tuple.0 * tuple.1
}
