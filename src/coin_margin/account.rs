use std::collections::BTreeMap;

use crate::client::*;
use crate::coin_margin::rest_model::*;
use crate::errors::*;
use crate::util::*;
#[derive(Clone)]
pub struct CoinAccount {
    pub client: Client,
    pub recv_window: u64,
}

impl CoinAccount {
    /// Get currently open orders
    pub async fn get_open_orders(&self, symbol: impl Into<String>) -> Result<Vec<Order>> {
        let mut params = vec![];
        params.push(("symbol", symbol.into()));
        let payload = build_signed_request_p(params, self.recv_window)?;
        self.client.get_signed("/dapi/v1/openOrders", &payload).await
    }

    pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let params: Vec<&str> = vec![];
        let payload = build_signed_request_p(params, self.recv_window)?;
        self.client.get_signed("/dapi/v1/openOrders", &payload).await
    }

    pub async fn account_information(&self) -> Result<AccountInformation> {
        let payload = build_signed_request(BTreeMap::<String, String>::new(), self.recv_window)?;
        self.client.get_signed_d("/dapi/v1/account", &payload).await
    }

    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::<String, String>::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed_d("/dapi/v1/balance", request.as_str()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::Binance, config::Config};

    use super::*;
    use dotenvy::dotenv;
    #[tokio::test]
    async fn test_coinm_account_open_order() {
        // setup logger
        dotenv().ok();
        let account: CoinAccount = Binance::new_with_env(&Config::default());
        let orders = account.get_open_orders("APTUSDT").await;
        println!("{:?}", orders);
        assert!(orders.is_ok());
    }

    #[tokio::test]
    async fn test_coinm_account_all_open_orders() {
        // setup logger
        dotenv().ok();
        let account: CoinAccount = Binance::new_with_env(&Config::default());
        let orders = account.get_all_open_orders().await;
        println!("{:?}", orders);
        assert!(orders.is_ok());
    }

    #[tokio::test]
    async fn test_coinm_account_information() {
        // setup logger
        dotenv().ok();
        let account: CoinAccount = Binance::new_with_env(&Config::default());
        let info = account.account_information().await;
        println!("{:?}", info);
        assert!(info.is_ok());
    }

    #[tokio::test]
    async fn test_coinm_account_balance() {
        // setup logger
        dotenv().ok();
        let account: CoinAccount = Binance::new_with_env(&Config::default());
        let balance = account.account_balance().await;
        println!("{:?}", balance);
        assert!(balance.is_ok());
    }
}
