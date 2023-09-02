pub mod candles;
pub mod plateform_status;
pub mod ticker;
pub mod tickers;
pub mod tickers_history;

const PUB_ENDPOINT: &str = "https://api-pub.bitfinex.com/v2";

pub trait BitfinexRequest {
    fn path(&self) -> String;

    fn url_params(&self) -> Option<String> {
        None
    }
}
