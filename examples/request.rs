use binance_dex_rs::{query, BinanceDexClient};
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let client = BinanceDexClient::new();

    let rune = "RUNE-B1A_BNB".to_string();

    // Trades request example
    let trade_page = client
        .query(query::Trades {
            symbol: Some(rune.as_str()),
            address: None,
            buyer_order_id: None,
            end_time: None,
            start_time: None,
            block_height: None,
            limit: None,
            offset: None,
            quote_asset: None,
            seller_order_id: None,
            side: None,
            total: None,
        })
        .await?;

    print!("{:#?}", trade_page);

    Ok(())
}
