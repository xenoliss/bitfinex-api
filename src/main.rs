use bitfinex_rs::public::{
    candles::{get_candles, CandleType, GetCandlesReq, GetCandlesReqFilter, SortType, TimeFrame},
    plateform_status::get_platform_status,
    ticker::{get_ticker, GetTickerReq},
    tickers::{get_tickers, GetTickersReq},
    tickers_history::{get_tickers_history, GetTickersHistoryReq, GetTickersHistoryReqFilter},
};

#[tokio::main]
async fn main() {
    let status = get_platform_status().await;
    println!("{status:?}");

    let ticker = get_ticker(GetTickerReq { symbol: "tBTCUSD" }).await;
    println!("{ticker:?}");

    let tickers = get_tickers(GetTickersReq::Only(vec!["tBTCUSD"])).await;
    println!("{tickers:?}");

    let tickers_history = get_tickers_history(GetTickersHistoryReq::Only {
        symbols: vec!["tBTCUSD"],
        filter: Some(GetTickersHistoryReqFilter {
            limit: Some(5),
            start: None,
            end: None,
        }),
    })
    .await;
    println!("{tickers_history:?}");

    let candles = get_candles(GetCandlesReq {
        candle_type: CandleType::FundingCurrency {
            time_frame: TimeFrame::FifteenMins,
            currency: "fUSD",
            period: 2,
        },
        filter: Some(GetCandlesReqFilter {
            sort_mts: Some(SortType::Desc),
            start_mts: None,
            end_mts: None,
            limit: Some(3),
        }),
    })
    .await;
    println!("{candles:?}");
}
