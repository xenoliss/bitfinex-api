use bitfinex_rs::{
    api::{
        platform_status::{PlatformStatus, PlatformStatusResp},
        query::AsyncQuery,
    },
    bitfinex::AsyncBitfinex,
};

#[tokio::main]
async fn main() {
    let api = AsyncBitfinex::default();
    let endpoint = PlatformStatus::builder().build().unwrap();
    let r: PlatformStatusResp = endpoint.query_async(&api).await.unwrap();
    println!("{r:?}");
}
