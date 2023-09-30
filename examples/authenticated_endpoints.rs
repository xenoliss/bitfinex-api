use bitfinex_api::{
    api::{
        authenticated::{
            funding::{
                active_funding_offers::{ActiveFundingOffers, ActiveFundingOffersResp},
                cancel_all_funding_offers::{CancelAllFundingOffers, CancelAllFundingOffersResp},
                cancel_funding_offer::{CancelFundingOffer, CancelFundingOfferResp},
                funding_credits::{FundingCredits, FundingCreditsResp},
                funding_info::{FundingInfo, FundingInfoResp},
                funding_loans::{FundingLoans, FundingLoansResp},
                submit_funding_offer::{SubmitFundingOffer, SubmitFundingOfferResp},
                types::FundingOfferType,
            },
            orders::{
                cancel_order::{CancelOrder, CancelOrderResp},
                cancel_orders::{CancelOrders, CancelOrdersResp, CancelOrdersType},
                retrieve_orders::{RetrieveOrders, RetrieveOrdersResp},
                retrieve_orders_by_symbol::{RetrieveOrdersBySymbol, RetrieveOrdersBySymbolResp},
                submit_order::{SubmitOrder, SubmitOrderResp},
                types::{OrderFlag, OrderType},
            },
            wallets::{Wallets, WalletsResp},
        },
        ignore::ignore,
        query::AsyncQuery,
    },
    bitfinex::AsyncBitfinex,
};

#[tokio::main]
async fn main() {
    let client = AsyncBitfinex::new_auth("YOUR_API_KEY", "YOUR_SECRET_KEY");

    let endpoint = Wallets::builder().build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: WalletsResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = SubmitFundingOffer::builder()
        .ty(FundingOfferType::Limit)
        .symbol("fUSD")
        .amount(150.)
        .rate(0.009)
        .period(2)
        .hidden(true)
        .build()
        .unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: SubmitFundingOfferResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = CancelAllFundingOffers::builder()
        .currency("USD")
        .build()
        .unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: CancelAllFundingOffersResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = ActiveFundingOffers::builder()
        .symbol("fUSD")
        .build()
        .unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: ActiveFundingOffersResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = FundingLoans::builder().symbol("fUSD").build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: FundingLoansResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = FundingCredits::builder().symbol("fUSD").build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: FundingCreditsResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = FundingInfo::builder().symbol("fUSD").build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: FundingInfoResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = CancelFundingOffer::builder().id(12345).build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: CancelFundingOfferResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = RetrieveOrders::builder().build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: RetrieveOrdersResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = RetrieveOrdersBySymbol::builder()
        .symbol("tBTCUSD")
        .build()
        .unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: RetrieveOrdersBySymbolResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = SubmitOrder::builder()
        .ty(OrderType::Market)
        .symbol("tBTCUSD")
        .amount(0.1)
        .price(1000.)
        .price_aux_limit(1111.)
        .flags(vec![OrderFlag::Hidden, OrderFlag::PostOnly])
        .build()
        .unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: SubmitOrderResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = CancelOrder::builder().id(12345).build().unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: CancelOrderResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");

    let endpoint = CancelOrders::builder()
        .cancel_orders_type(CancelOrdersType::All)
        .build()
        .unwrap();
    ignore(endpoint).query_async(&client).await.unwrap();
    // let r: CancelOrdersResp = endpoint.query_async(&client).await.unwrap();
    // println!("{r:#?}");
}
