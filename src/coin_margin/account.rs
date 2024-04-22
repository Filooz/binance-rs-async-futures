use std::collections::BTreeMap;

// use rust_decimal::Decimal;
use serde_with::skip_serializing_none;

use crate::client::*;
use crate::coin_margin::rest_model::*;
use crate::errors::*;
use crate::futures::rest_model::PositionSide;
use crate::rest_model::OrderType;
use crate::util::*;

use super::ws_model::WorkingType;
#[derive(Clone)]
pub struct CoinAccount {
    pub client: Client,
    pub recv_window: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub position_side: Option<PositionSide>,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: Option<String>,
    pub price: Option<f64>,
    #[serde(rename = "newClientOrderId")]
    pub new_client_order_id: Option<String>,
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<f64>,
    #[serde(rename = "closePosition")]
    pub close_position: Option<String>,
    #[serde(rename = "activationPrice")]
    pub activation_price: Option<f64>,
    #[serde(rename = "callbackRate")]
    pub callback_rate: Option<f64>,
    #[serde(rename = "workingType")]
    pub working_type: Option<WorkingType>,
    #[serde(rename = "priceProtect")]
    pub price_protect: Option<String>,
    #[serde(rename = "newOrderRespType")]
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// Order Cancellation and Replace Request
/// Cancels an existing order and places a new order on the same symbol.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceRequest {
    pub orig_client_order_id: Option<String>,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: Option<f64>,
    // #[serde(rename = "type")]
    // pub order_type: OrderType,
    pub price: Option<f64>,
    // pub recv_window: Option<u64>,
    // pub time_in_force: Option<TimeInForce>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NewOrderRespType {
    ACK,
    RESULT,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinmBrackets {
    #[serde(alias = "pair")]
    pub symbol: String,
    pub brackets: Vec<CoinmBracket>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoinmBracket {
    pub bracket: u8,
    pub initial_leverage: u64,
    pub qty_cap: u64,
    pub qty_floor: u64,
    #[serde(rename = "maintMarginRatio")]
    pub maintenance_margin_ratio: f64,
    pub cum: f64,
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
    use reqwest::Method;
    // use serde_json::Value;
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

    #[tokio::test]
    async fn test_coinm_brackets() {
        // setup logger
        dotenv().ok();
        let client = GenericClient::new_with_secrets().unwrap();
        let host = "https://dapi.binance.com";
        let url = "/dapi/v2/leverageBracket";
        let res = client
            .get_signed_p::<Vec<CoinmBrackets>, String>(host, url, None, 1500)
            .await;
        assert!(res.is_ok());

        let res = client
            .send_request::<String, Vec<CoinmBrackets>>(host, url, Method::GET, None, 1500, true)
            .await;
        assert!(res.is_ok());
    }
}
