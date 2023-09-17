# bitfinex-rs

Another unofficial Rust Library for the [Bitfinex API V2](https://docs.bitfinex.com/docs#).


## Installation

Add the lib as a project dependency in you `Cargo.toml`.

```
[dependencies]
bitfinex-rs = { git = "https://github.com/xenoliss/bitfinex-rs" }
```


## Basic Usage

The library provides the necessary building blocks to interact with the Bitfinex API V2. It mostly exposes three main traits that you can implement on your types to build logic that fits your exact needs:

- The `Client` / `AsyncClient` traits can be implemented on tour types to turn them into working instance that can communcicate with the API. Default `Bitfinex`/`AsyncBitfinex` clients are already implemented.

- The `Endpoint` trait can be implemented on your types to turn them into actual endpoints that your application needs to interact with. Not all the endpoints are currently implemented but it's really easy to add new ones or adapt existing ones to fit your exact needs.

- The `AsyncQuery` / `Query` traits are implemented on all types that implement `Endpoint` and expose the `query` / `query_async` methods in which the `Client` / `AsyncClient` are injected to perform the requests.

Here is an example of querying the [Ticker](https://docs.bitfinex.com/reference/rest-public-ticker) endpoint using the default implementations provided by the lib:

```rs
#[tokio::main]
async fn main() {
    // 1. Instanciate an `AsyncClient`.
    let client = AsyncBitfinex::default();

    // 2. Build the endpoint using the builder pattern.
    // NOTE: The builder pattern allows to easily add required/optional parameters to each endpoint.
    let endpoint = PlatformStatus::builder().build().unwrap();

    // 3. Perform the query against the endpoint.
    // NOTE: The returned type needs to be explicitly set by the caller. This is intended as it allows
    // for a greater flexibility for the lib user. Default returned types (like here with the `PlatformStatusResp`)
    // cna be used when they are implemented.
    let r: PlatformStatusResp = endpoint.query_async(&client).await.unwrap();
}

```

Here is another example that queries the [Submit Funding Offer](https://docs.bitfinex.com/reference/rest-auth-submit-funding-offer) endpoint:

```rs
#[tokio::main]
async fn main() {
    // 1. Instanciate an `AsyncClient`, this time providing credentials as we want to query an authenticated endpoint.
    let client = AsyncBitfinex::new_auth(dotenv!("API_KEY"), dotenv!("SECRET_KEY"));

    // 2. Build the endpoint using the builder pattern.
    let endpoint = SubmitFundingOffer::builder()
        .ty(FundingOrderType::Limit)
        .symbol("fUSD")
        .amount(150.)
        .rate(0.009)
        .period(2)
        .build()
        .unwrap();

    // 3. Perform the query against the endpoint.
    let r: SubmitFundingOfferResp = endpoint.query_async(&client).await.unwrap();
}
```

That's it ! That's the same pattern for all the endpoints that are implemented in the lib.

## Advanced Usage

You might want to implement your own endpoints and your own return type for them (PRs are welcomed!). 

Let's see how we can simply create a new `Endpoint` from scratch. We're going to implement the [Liquidations](https://docs.bitfinex.com/reference/rest-public-liquidations) endpoint as an example.

- First we need to create the endpoint type:
```rs

// Create the endpoint struct which gather all the query / body params that are needed to perform
// the query. The optional parameters should be wrapped in `Option` types.
// NOTE: For convenience the `derive_builder` crate is used to easily build such types.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Liquidations {
    #[builder(default)]
    sort: Option<Sort>,
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
    #[builder(default)]
    limit: Option<u64>,
}

impl Liquidations {
    pub fn builder() -> LiquidationsBuilder {
        LiquidationsBuilder::default()
    }
}
```

- Then we need to implement the `Endpoint` trait:
```rs
// Implement the `Endpoint` trait on our type.
impl Endpoint for Liquidations {

    // The endpoint is a GET.
    fn method(&self) -> Method {
        Method::GET
    }

    // Give the endpoint path.
    fn endpoint(&self) -> String {
        String::from("v2/liquidations/hist")
    }   

    // Provide the query parameters associated with this endpoint.
    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params
            .push_opt("sort", self.sort.map(|sort| sort as i8))
            .push_opt("start", self.start)
            .push_opt("end", self.end)
            .push_opt("limit", self.limit);
        params
    }
}
```

That's it ! We created a new endpoint that can now be used in our application to retrieve the liquidations.

In addition to creating endpoints you might want to create the return type that fits your own needs. Let's see how we can create our own `LiquidationsResp` that only contains the fields we are interested in. For this example let's say we only need the `POS_ID`, `SYMBOL` and `AMOUNT` fields from each liquidation item:

- Start by declaring the return type with the needed fields:
```rs
#[derive(Debug)]
pub struct LiquidationResp {
    pub pos_id: u64,
    pub symbol: String,
    pub amount: f64,
}
```

- Then due to how Bitfinex API returns the repsonse we need to manually implement the `serde::Deserialize` trait:
```rs
impl<'de> Deserialize<'de> for LiquidationResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 1. Define the raw payload that is returned by the API.
        // NOTE: This is a tuple struct! We need this because Bitfinex API responses are arrays
        // of values instead of clean JSON objects.
        #[derive(Debug, Deserialize)]
        struct LiquidationRawResp(
            String,
            u64,
            u64,
            Option<()>,
            String,
            f64,
            f64,
            Option<()>,
            u8,
            u8,
            Option<()>,
            Option<f64>,
        );

        // 2. Implement the `From` trait to convert the raw type (`LiquidationRawResp` here) 
        // into the targeted one (`LiquidationResp` here).
        impl From<LiquidationRawResp> for LiquidationResp {
            fn from(value: LiquidationRawResp) -> Self {

                // .1 Extract the desired fields.
                let LiquidationRawResp(
                    _,
                    pos_id,
                    _,
                    _,
                    symbol,
                    amount,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) = value;

                // .2 Build our response type.
                Self {
                    pos_id,
                    symbol,
                    amount,
                }
            }
        }

        // 3. Deserialize the JSON payload into our `LiquidationRawResp`.
        // NOTE: Due to the shape of the returned JSON in reality, this isn't exactly what's done 
        // if you look at the source code, but that's not important for the example.
        let raw = LiquidationRawResp::deserialize(deserializer)?;

        // 4. Finally convert the `LiquidationRawResp` into a `LiquidationResp`.
        Ok(raw.into())
    }
}
```

- Finally for convenience we can create a wrapper type like so:
```rs
pub type LiquidationsResp = Vec<LiquidationResp>;
```

- We can now use our endpoint allong with our newly created returned type:
```rs
#[tokio::main]
async fn main() {
    // 1. Instanciate an `AsyncClient`.
    let client = AsyncBitfinex::default();

    // 2. Build the endpoint using the builder pattern.
    let endpoint = Liquidations::builder()
        .ty(FundingOrderType::Limit)
        .symbol("fUSD")
        .amount(150.)
        .rate(0.009)
        .period(2)
        .build()
        .unwrap();

    // 3. Perform the query against the endpoint.
    let r: LiquidationsResp = endpoint.query_async(&client).await.unwrap();
}
```

Feel free to dig in the individual endpoints source code (in the `api/public` and `api/authenticated` folders) to see how the implementations vary depending on the endpoint path, query and body parameters.

## Implemented Endpoints

### Public Endpoints
- :white_check_mark: [Platform Status](https://docs.bitfinex.com/reference/rest-public-platform-status)
    - Endpoint `PlatformStatus`
    - Return `PlatformStatus`

- :white_check_mark: [Ticker](https://docs.bitfinex.com/reference/rest-public-ticker)
    - Endpoint `Ticker`
    - Return `TickerResp`

- :white_check_mark: [Tickers](https://docs.bitfinex.com/reference/rest-public-tickers)
    - Endpoint `Tickers`
    - Return `TickersResp`

- :white_check_mark: [Tickers History](https://docs.bitfinex.com/reference/rest-public-tickers-history)
    - Endpoint `TickersHistory`
    - Return `TickersHistoryResp`

- :white_check_mark: [Trades](https://docs.bitfinex.com/reference/rest-public-trades)
    - Endpoint `Trades`
    - Return `TradesResp`

- :white_check_mark: [Book](https://docs.bitfinex.com/reference/rest-public-book)
    - Endpoint `Book` / `RawBook`
    - Return `BookResp` / `RawBookResp`

- :white_check_mark: [Stats](https://docs.bitfinex.com/reference/rest-public-stats)
    - Endpoint `Stats`
    - Return `LastStatsResp` or `HistStatsResp`

- :white_check_mark: [Candles](https://docs.bitfinex.com/reference/rest-public-candles)
    - Endpoint `Candles`
    - Return `LastCandlesResp` or `HistCandlesResp`

- :white_check_mark: [Derivatives Status](https://docs.bitfinex.com/reference/rest-public-derivatives-status)
    - Endpoint `DerivativeStatus`
    - Return `DerivativeStatusResp`

- :white_check_mark: [Derivatives Status History](https://docs.bitfinex.com/reference/rest-public-derivatives-status-history)
    - Endpoint `DerivativesStatusHistory`
    - Return `DerivativesStatusHistoryResp`

- :white_check_mark: [Liquidations](https://docs.bitfinex.com/reference/rest-public-liquidations)
    - Endpoint `Liquidations`
    - Return `LiquidationsResp`

- :white_check_mark: [Leaderboards](https://docs.bitfinex.com/reference/rest-public-rankings)
    - Endpoint `Leaderboards`
    - Return `LastLeaderBoardResp` or `HistLeaderBoardsResp`

- :white_check_mark: [Funding Statistics](https://docs.bitfinex.com/reference/rest-public-funding-stats)
    - Endpoint `FundingStatistics`
    - Return `FundingStatisticsResp`

- :black_square_button: [Configs](https://docs.bitfinex.com/reference/rest-public-conf)

### Authenticated Endpoints

- :white_check_mark: [Wallets](https://docs.bitfinex.com/reference/rest-auth-wallets)
    - Endpoint `Wallets`
    - Return `WalletsResp`

- :white_check_mark: [Retrieve Orders](https://docs.bitfinex.com/reference/rest-auth-retrieve-orders)
    - Endpoint `RetrieveOrders`
    - Return `RetrieveOrdersResp`
- :white_check_mark: [Retrieve Orders (by symbol)](https://docs.bitfinex.com/reference/rest-auth-retrieve-orders-by-symbol)
    - Endpoint `RetrieveOrdersBySymbol`
    - Return `RetrieveOrdersBySymbolResp`
- :white_check_mark: [Submit Order](https://docs.bitfinex.com/reference/rest-auth-submit-order)
    - Endpoint `SubmitOrder`
    - Return `SubmitOrderResp`
- :black_square_button: [Update Order](https://docs.bitfinex.com/reference/rest-auth-update-order)
- :white_check_mark: [Cancel Order](https://docs.bitfinex.com/reference/rest-auth-cancel-order)
    - Endpoint `CancelOrder`
    - Return `CancelOrderResp`
- :white_check_mark: [Cancel Orders (multiple)](https://docs.bitfinex.com/reference/rest-auth-cancel-orders-multiple)
    - Endpoint `CancelOrders`
    - Return `CancelOrdersResp`
- :black_square_button: [Order Multi-OP](https://docs.bitfinex.com/reference/rest-auth-order-multi)
- :black_square_button: [Orders History](https://docs.bitfinex.com/reference/rest-auth-orders-history)
- :black_square_button: [Orders History (by symbol)](https://docs.bitfinex.com/reference/rest-auth-orders-history-by-symbol)
- :black_square_button: [Order Trades](https://docs.bitfinex.com/reference/rest-auth-order-trades)
- :black_square_button: [Trades](https://docs.bitfinex.com/reference/rest-auth-trades)
- :black_square_button: [Trades (by symbol)](https://docs.bitfinex.com/reference/rest-auth-trades-by-symbol)
- :black_square_button: [OTC Orders History](https://docs.bitfinex.com/reference/otc-orders-history)
- :black_square_button: [Ledgers](https://docs.bitfinex.com/reference/rest-auth-ledgers)
- :black_square_button: [Margin Info](https://docs.bitfinex.com/reference/rest-auth-info-margin)
- :black_square_button: [Retrieve Positions](https://docs.bitfinex.com/reference/rest-auth-positions)
- :black_square_button: [Claim Position](https://docs.bitfinex.com/reference/rest-auth-position-claim)
- :black_square_button: [Increase Position](https://docs.bitfinex.com/reference/rest-auth-position-increase)
- :black_square_button: [Increase Position Info](https://docs.bitfinex.com/reference/rest-auth-increase-position-info)
- :black_square_button: [Positions History](https://docs.bitfinex.com/reference/rest-auth-positions-hist)
- :black_square_button: [Positions Snapshot](https://docs.bitfinex.com/reference/rest-auth-positions-snap)
- :black_square_button: [Positions Audit](https://docs.bitfinex.com/reference/rest-auth-positions-audit)
- :black_square_button: [Derivative Position Collateral](https://docs.bitfinex.com/reference/rest-auth-deriv-pos-collateral-set)
- :black_square_button: [Derivative Position Collateral Limits](https://docs.bitfinex.com/reference/rest-auth-calc-deriv-collateral-limits)
- :white_check_mark: [Active Funding Offers](https://docs.bitfinex.com/reference/rest-auth-funding-offers)
    - Endpoint `ActiveFundingOffers`
    - Return `ActiveFundingOffersResp`
- :white_check_mark: [Submit Funding Offer](https://docs.bitfinex.com/reference/rest-auth-submit-funding-offer)
    - Endpoint `SubmitFundingOffer`
    - Return `SubmitFundingOfferResp`
- :white_check_mark: [Cancel Funding Offer](https://docs.bitfinex.com/reference/rest-auth-cancel-funding-offer)
    - Endpoint `CancelFundingOffer`
    - Return `CancelFundingOfferResp`
- :white_check_mark: [Cancel All Funding Offers](https://docs.bitfinex.com/reference/rest-auth-cancel-all-funding-offers)
    - Endpoint `CancelAllFundingOffers`
    - Return `CancelAllFundingOffersResp`
- :black_square_button: [Funding Close](https://docs.bitfinex.com/reference/rest-auth-funding-close)
- :black_square_button: [Funding Auto-renew](https://docs.bitfinex.com/reference/rest-auth-funding-auto-renew)
- :black_square_button: [Keep Funding](https://docs.bitfinex.com/reference/rest-auth-keep-funding)
- :black_square_button: [Funding Offers History](https://docs.bitfinex.com/reference/rest-auth-funding-offers-hist)
- :black_square_button: [Funding Loans](https://docs.bitfinex.com/reference/rest-auth-funding-loans)
- :black_square_button: [Funding Loans History](https://docs.bitfinex.com/reference/rest-auth-funding-loans-hist)
- :black_square_button: [Funding Credits](https://docs.bitfinex.com/reference/rest-auth-funding-credits)
- :black_square_button: [Funding Credits History](https://docs.bitfinex.com/reference/rest-auth-funding-credits-hist)
- :black_square_button: [Funding Trades](https://docs.bitfinex.com/reference/rest-auth-funding-trades-hist)
- :black_square_button: [Funding Info](https://docs.bitfinex.com/reference/rest-auth-info-funding)
- :black_square_button: [User Info](https://docs.bitfinex.com/reference/rest-auth-info-user)
- :black_square_button: [Summary](https://docs.bitfinex.com/reference/rest-auth-summary)
- :black_square_button: [Login History](https://docs.bitfinex.com/reference/rest-auth-logins-hist)
- :black_square_button: [Key Permissions](https://docs.bitfinex.com/reference/key-permissions)
- :black_square_button: [Generate Token](https://docs.bitfinex.com/reference/generate-token)
- :black_square_button: [Changelog](https://docs.bitfinex.com/reference/rest-auth-audit-hist)
- :black_square_button: [Transfer Between Wallets](https://docs.bitfinex.com/reference/rest-auth-transfer)
- :black_square_button: [Deposit Address](https://docs.bitfinex.com/reference/rest-auth-deposit-address)
- :black_square_button: [Generate Invoice](https://docs.bitfinex.com/reference/rest-auth-deposit-invoice)
- :black_square_button: [LNX Invoice Payments](https://docs.bitfinex.com/reference/lnx-invoice-payments)
- :black_square_button: [Withdrawal](https://docs.bitfinex.com/reference/rest-auth-withdraw)
- :black_square_button: [Movements](https://docs.bitfinex.com/reference/rest-auth-movements)
- :black_square_button: [Movement info](https://docs.bitfinex.com/reference/movement-info)
- :black_square_button: [Alert List](https://docs.bitfinex.com/reference/rest-auth-alerts)
- :black_square_button: [Alert Set](https://docs.bitfinex.com/reference/rest-auth-alert-set)
- :black_square_button: [Alert Delete](https://docs.bitfinex.com/reference/rest-auth-alert-del)
- :black_square_button: [Balance Available for Orders/Offers](https://docs.bitfinex.com/reference/rest-auth-calc-order-avail)
- :black_square_button: [User Settings Write](https://docs.bitfinex.com/reference/rest-auth-settings-set)
- :black_square_button: [User Settings Read](https://docs.bitfinex.com/reference/rest-auth-settings)
- :black_square_button: [User Settings Delete](https://docs.bitfinex.com/reference/rest-auth-settings-del)
