use bitfinex_rs::{
    api::{
        authenticated::{
            active_funding_offers::{ActiveFundingOffers, ActiveFundingOffersResp},
            cancel_all_funding_offers::{CancelAllFundingOffers, CancelAllFundingOffersResp},
            cancel_funding_offer::{CancelFundingOffer, CancelFundingOfferResp},
            submit_funding_offer::{FundingOrderType, SubmitFundingOffer, SubmitFundingOfferResp},
            wallets::{Wallets, WalletsResp},
        },
        query::AsyncQuery,
    },
    bitfinex::AsyncBitfinex,
};

#[tokio::main]
async fn main() {
    let client = AsyncBitfinex::new_auth("YOUR_API_KEY", "YOUR_SECRET_KEY");

    let endpoint = Wallets::builder().build().unwrap();
    let r: WalletsResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = SubmitFundingOffer::builder()
        .ty(FundingOrderType::Limit)
        .symbol("fUSD")
        .amount(150.)
        .rate(0.009)
        .period(2)
        .build()
        .unwrap();
    let r: SubmitFundingOfferResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = CancelAllFundingOffers::builder()
        .currency("USD")
        .build()
        .unwrap();
    let r: CancelAllFundingOffersResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = ActiveFundingOffers::builder()
        .symbol("fUSD")
        .build()
        .unwrap();
    let r: ActiveFundingOffersResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");

    let endpoint = CancelFundingOffer::builder().id(12345).build().unwrap();
    let r: CancelFundingOfferResp = endpoint.query_async(&client).await.unwrap();
    println!("{r:#?}");
}
