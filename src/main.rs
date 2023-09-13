use bitfinex_rs::{
    api::{
        book::{
            book::{Book, BookResp, Precision},
            common::Len,
            raw_book::{RawBook, RawBookResp},
        },
        candles::{AvailableCandles, Candles, HistCandlesResp, LastCandlesResp, TimeFrame},
        common::{Section, Symbols},
        derivative_status::{DerivativesStatus, DerivativesStatusesResp},
        platform_status::{PlatformStatus, PlatformStatusResp},
        query::AsyncQuery,
        stats::{HistStatsResp, KeyArgs, LastStatsResp, Side, Stats},
        ticker::{Ticker, TickerResp},
        tickers::{Tickers, TickersResp},
        tickers_history::{TickersHistory, TickersHistoryResp},
        trades::{Trades, TradesResp},
    },
    bitfinex::AsyncBitfinex,
};

#[tokio::main]
async fn main() {
    let client = AsyncBitfinex::default();

    let endpoint = PlatformStatus::builder().build().unwrap();
    let r: PlatformStatusResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Ticker::builder().symbol("tBTCUSD").build().unwrap();
    let r: TickerResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Tickers::builder()
        .symbols(Symbols::Only(vec!["tBTCUSD"]))
        .build()
        .unwrap();
    let r: TickersResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = TickersHistory::builder()
        .symbols(Symbols::Only(vec!["tBTCUSD", "tARBF0:USTF0"]))
        .limit(1)
        .start(1694538014999)
        .end(1694538015000)
        .build()
        .unwrap();
    let r: TickersHistoryResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Trades::builder()
        .symbol("tBTCUSD")
        .limit(2)
        .start(1694538015000)
        .build()
        .unwrap();
    let r: TradesResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Book::builder()
        .symbol("fUSD")
        .precision(Precision::P0)
        .len(Len::One)
        .build()
        .unwrap();
    let r: BookResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = RawBook::builder()
        .symbol("tBTCUSD")
        .len(Len::One)
        .build()
        .unwrap();
    let r: RawBookResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Stats::builder()
        .key_args(KeyArgs::PosSize {
            sym: "tBTCUSD",
            side: Side::Long,
        })
        .section(Section::Last)
        .build()
        .unwrap();
    let r: LastStatsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Stats::builder()
        .key_args(KeyArgs::VolOneDay { platform: "BFX" })
        .section(Section::Hist)
        .limit(5)
        .build()
        .unwrap();
    let r: HistStatsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Candles::builder()
        .candles(AvailableCandles::FundingCandles {
            time_frame: TimeFrame::FifteenMins,
            currency: "fUSD",
            period: 120,
        })
        .section(Section::Last)
        .build()
        .unwrap();
    let r: LastCandlesResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = Candles::builder()
        .candles(AvailableCandles::AggregateFundingCandles {
            time_frame: TimeFrame::FifteenMins,
            currency: "fUSD",
            aggregation: 30,
            period_start: 2,
            period_end: 30,
        })
        .section(Section::Hist)
        .limit(2)
        .build()
        .unwrap();
    let r: HistCandlesResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");

    let endpoint = DerivativesStatus::builder()
        .keys(Symbols::All)
        .build()
        .unwrap();
    let r: DerivativesStatusesResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:?}");
}
