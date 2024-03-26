use crate::errors::*;
use crate::{client::*,
            futures::rest_model::{FundingRate, HistoryQuery}};

#[derive(Clone)]
pub struct CoinmMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl CoinmMarket {
    pub async fn get_funding_rate<S1, S3, S4, S5>(
        &self,
        symbol: S1,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<FundingRate>>
    where
        S1: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        self.client
            .get_signed_p(
                "/dapi/v1/fundingRate",
                Some(HistoryQuery {
                    start_time: start_time.into(),
                    end_time: end_time.into(),
                    limit: limit.into(),
                    symbol: symbol.into(),
                    from_id: None,
                    interval: None,
                    period: None,
                }),
                self.recv_window,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Binance;

    #[tokio::test]
    async fn test_get_funding_rate() {
        // setup logger
        let _ = env_logger::builder().is_test(true).try_init();
        let market: CoinmMarket = Binance::new(None, None);
        let limit: u16 = 10;
        let funding_rate = market.get_funding_rate("BTCUSD_PERP", None, None, limit).await;
        println!("{:?}", funding_rate);
        assert!(funding_rate.is_ok());
    }
}
