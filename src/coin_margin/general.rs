use crate::client::*;
use crate::errors::*;
use crate::rest_model::ServerTime;
use crate::rest_model::Success;

use super::rest_model::ExchangeInformation;

#[derive(Clone)]
pub struct CoinGeneral {
    pub client: Client,
    pub recv_window: u64,
}

impl CoinGeneral {
    pub async fn ping(&self) -> Result<String> {
        let _: Success = self.client.get("/dapi/v1/ping", None).await?;
        Ok("pong".into())
    }

    pub async fn get_server_time(&self) -> Result<ServerTime> { self.client.get_p("/dapi/v1/time", None).await }

    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p("/dapi/v1/exchangeInfo", None).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::Binance, config::Config};

    use super::*;

    #[tokio::test]
    async fn test_coin_general_ping() {
        let general: CoinGeneral = Binance::new_with_env(&Config::default());
        let pong = general.ping().await;
        println!("{:?}", pong);
        assert!(pong.is_ok());
    }

    #[tokio::test]
    async fn test_coin_general_server_time() {
        let _ = env_logger::builder().is_test(true).try_init();

        let general: CoinGeneral = Binance::new(None, None);
        let server_time = general.get_server_time().await;
        println!("{:?}", server_time);
        assert!(server_time.is_ok());
    }

    #[tokio::test]
    async fn test_coin_general_exchange_info() {
        let _ = env_logger::builder().is_test(true).try_init();

        let general: CoinGeneral = Binance::new(None, None);
        let exchange_info = general.exchange_info().await;
        println!("{:?}", exchange_info);
        assert!(exchange_info.is_ok());
    }
}
