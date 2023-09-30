use bitfinex_api::{
    api::{
        common::{Section, Symbols, TimeFrame},
        public::{
            book::{
                book::{Book, BookResp, Precision},
                common::Len,
                raw_book::{RawBook, RawBookResp},
            },
            candles::{AvailableCandles, Candles, HistCandlesResp, LastCandlesResp},
            derivative_status::{DerivativesStatus, DerivativesStatusResp},
            derivative_status_history::{DerivativesStatusHistory, DerivativesStatusHistoryResp},
            funding_statistics::{FundingStatistics, FundingStatisticsResp},
            leaderboards::{HistLeaderBoardsResp, Key, Leaderboards},
            liquidations::{Liquidations, LiquidationsResp},
            platform_status::{PlatformStatus, PlatformStatusResp},
            stats::{HistStatsResp, KeyArgs, LastStatsResp, Side, Stats},
            ticker::{Ticker, TickerResp},
            tickers::{Tickers, TickersResp},
            tickers_history::{TickersHistory, TickersHistoryResp},
            trades::{Trades, TradesResp},
        },
        query::AsyncQuery,
    },
    bitfinex::AsyncBitfinex,
};

#[tokio::main]
async fn main() {
    let client = AsyncBitfinex::default();

    let endpoint = PlatformStatus::builder().build().unwrap();
    let r: PlatformStatusResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Ticker::builder().symbol("tBTCUSD").build().unwrap();
    let r: TickerResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Tickers::builder()
        .symbols(Symbols::Only(vec!["tBTCUSD"]))
        .build()
        .unwrap();
    let r: TickersResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = TickersHistory::builder()
        .symbols(Symbols::Only(vec!["tBTCUSD", "tARBF0:USTF0"]))
        .limit(1)
        .start(1694538014999)
        .end(1694538015000)
        .build()
        .unwrap();
    let r: TickersHistoryResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Trades::builder()
        .symbol("tBTCUSD")
        .limit(2)
        .start(1694538015000)
        .build()
        .unwrap();
    let r: TradesResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Book::builder()
        .symbol("fUSD")
        .precision(Precision::P0)
        .len(Len::One)
        .build()
        .unwrap();
    let r: BookResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = RawBook::builder()
        .symbol("tBTCUSD")
        .len(Len::One)
        .build()
        .unwrap();
    let r: RawBookResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Stats::builder()
        .key_args(KeyArgs::PosSize {
            sym: "tBTCUSD",
            side: Side::Long,
        })
        .section(Section::Last)
        .build()
        .unwrap();
    let r: LastStatsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Stats::builder()
        .key_args(KeyArgs::VolOneDay { platform: "BFX" })
        .section(Section::Hist)
        .limit(5)
        .build()
        .unwrap();
    let r: HistStatsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

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
    println!("{r:#?}");

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
    println!("{r:#?}");

    let endpoint = DerivativesStatus::builder()
        .keys(Symbols::Only(vec!["tBTCF0:USTF0", "tETHF0:USTF0"]))
        .build()
        .unwrap();
    let r: DerivativesStatusResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = DerivativesStatusHistory::builder()
        .key("tBTCF0:USTF0")
        .limit(1)
        .build()
        .unwrap();
    let r: DerivativesStatusHistoryResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Liquidations::builder().limit(3).build().unwrap();
    let r: LiquidationsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = Leaderboards::builder()
        .key(Key::Plr)
        .time_frame(TimeFrame::OneMonth)
        .symbol("tGLOBAL:USD")
        .section(Section::Hist)
        .limit(5)
        .build()
        .unwrap();
    let r: HistLeaderBoardsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = FundingStatistics::builder()
        .symbol("fUSD")
        .limit(3)
        .build()
        .unwrap();
    let r: FundingStatisticsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");
}
