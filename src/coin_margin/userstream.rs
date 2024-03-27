use crate::client::*;
use crate::errors::*;
use crate::futures::rest_model::OrderType;
use crate::futures::rest_model::PositionSide;
use crate::rest_model::Success;
use crate::rest_model::UserDataStream;

static USER_DATA_STREAM: &str = "/dapi/v1/listenKey";

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl UserStream {
    pub async fn start(&self) -> Result<UserDataStream> { self.client.post(USER_DATA_STREAM, None).await }

    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(USER_DATA_STREAM, listen_key, None).await
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client.delete(USER_DATA_STREAM, listen_key, None).await
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "e")]
pub enum UserStreamEvent {
    MarginCall(MarginCallEvent),
    AccountUpdate(AccountUpdateEvent),
    OrderTradeUpdate(OrderTradeUpdateEvent),
}

use rust_decimal::Decimal;
use serde;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::primitive::str;

use super::rest_model::OrderSide;
use super::rest_model::OrderStatus;
use super::rest_model::TimeInForce;
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarginCallEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "i")]
    pub account_alias: String,
    #[serde(rename = "cw", with = "rust_decimal::serde::str")]
    pub cross_wallet_balance: Decimal,
    #[serde(rename = "p")]
    pub positions: Vec<MarginCallPosition>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarginCallPosition {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "pa", with = "rust_decimal::serde::str")]
    pub position_amount: Decimal,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw", with = "rust_decimal::serde::str")]
    pub isolated_wallet: Decimal,
    #[serde(rename = "mp", with = "rust_decimal::serde::str")]
    pub mark_price: Decimal,
    #[serde(rename = "up", with = "rust_decimal::serde::str")]
    pub unrealized_pnl: Decimal,
    #[serde(rename = "mm", with = "rust_decimal::serde::str")]
    pub maintenance_margin_required: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "i")]
    pub account_alias: String,
    #[serde(rename = "a")]
    pub update_data: UpdateData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateData {
    #[serde(rename = "m")]
    pub event_reason_type: String,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", with = "rust_decimal::serde::str")]
    pub wallet_balance: Decimal,
    #[serde(rename = "cw", with = "rust_decimal::serde::str")]
    pub cross_wallet_balance: Decimal,
    #[serde(rename = "bc", with = "rust_decimal::serde::str")]
    pub balance_change: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", with = "rust_decimal::serde::str")]
    pub position_amount: Decimal,
    #[serde(rename = "ep", with = "rust_decimal::serde::str")]
    pub entry_price: Decimal,
    #[serde(rename = "bep", with = "rust_decimal::serde::str")]
    pub break_even_price: Decimal,
    #[serde(rename = "cr", with = "rust_decimal::serde::str")]
    pub accumulated_realized: Decimal,
    #[serde(rename = "up", with = "rust_decimal::serde::str")]
    pub unrealized_pnl: Decimal,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw", with = "rust_decimal::serde::str")]
    pub isolated_wallet: Decimal,
    #[serde(rename = "ps")]
    pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTradeUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "i")]
    pub account_alias: String,
    #[serde(rename = "o")]
    pub order: OrderDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetail {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,

    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,

    #[serde(rename = "q", with = "rust_decimal::serde::str")]
    pub original_quantity: Decimal,

    #[serde(rename = "p", with = "rust_decimal::serde::str")]
    pub original_price: Decimal,

    #[serde(rename = "ap", with = "rust_decimal::serde::str")]
    pub average_price: Decimal,

    #[serde(rename = "sp", with = "rust_decimal::serde::str")]
    pub stop_price: Decimal,

    #[serde(rename = "x")]
    pub execution_type: ExecutionType,

    #[serde(rename = "X")]
    pub order_status: OrderStatus,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l", with = "rust_decimal::serde::str")]
    pub last_filled_quantity: Decimal,

    #[serde(rename = "z", with = "rust_decimal::serde::str")]
    pub filled_accumulated_quantity: Decimal,

    #[serde(rename = "L", with = "rust_decimal::serde::str")]
    pub last_filled_price: Decimal,

    #[serde(rename = "ma")]
    pub margin_asset: Option<String>,

    #[serde(rename = "N")]
    pub commission_asset: Option<String>,

    #[serde(rename = "n", with = "rust_decimal::serde::str")]
    pub commission: Decimal,

    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "rp", with = "rust_decimal::serde::str")]
    pub realized_profit: Decimal,

    #[serde(rename = "b", with = "rust_decimal::serde::str")]
    pub bid_quantity: Decimal,

    #[serde(rename = "a", with = "rust_decimal::serde::str")]
    pub ask_quantity: Decimal,

    #[serde(rename = "m")]
    pub is_maker: bool,

    #[serde(rename = "R")]
    pub is_reduce_only: bool,

    #[serde(rename = "wt")]
    pub working_type: WorkingType,

    #[serde(rename = "ot")]
    pub original_order_type: OrderType,

    #[serde(rename = "ps")]
    pub position_side: PositionSide,

    #[serde(rename = "cp")]
    pub close_all: bool,

    #[serde(rename = "AP", with = "rust_decimal::serde::str")]
    pub activation_price: Decimal,

    #[serde(rename = "cr", with = "rust_decimal::serde::str")]
    pub callback_rate: Decimal,

    #[serde(rename = "pP")]
    pub price_protection: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    New,
    Canceled,
    Calculated,
    Expired,
    Trade,
    Amendment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    ContractPrice,
    MarkPrice,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dotenvy::dotenv;
    use serde_json::json;

    use crate::coin_margin::userstream::UserStreamEvent;
    use crate::{api::Binance, config::Config};

    #[tokio::test]
    async fn test_userstream_start() {
        use super::*;

        dotenv().ok();

        let userstream: UserStream = Binance::new_with_env(&Config::default());
        let start = userstream.start().await;
        println!("{:?}", start);
        assert!(start.is_ok());
        let key = start.unwrap().listen_key.clone();

        let keep_alive = userstream.keep_alive(&key).await;
        println!("{:?}", keep_alive);
        assert!(keep_alive.is_ok());

        let close = userstream.close(&key).await;
        println!("{:?}", close);
        assert!(close.is_ok());
    }

    #[tokio::test]
    async fn test_userstream_deserialize_margin_call() {
        let json = json!({
            "e":"MARGIN_CALL",        // Event Type
            "E":1587727187525 as i64,        // Event Time
            "i": "SfsR",              // Account Alias
            "cw":"3.16812045",        // Cross Wallet Balance. Only pushed with crossed position margin call
            "p":[                     // Position(s) of Margin Call
              {
                "s":"BTCUSD_200925",  // Symbol
                "ps":"LONG",          // Position Side
                "pa":"132",           // Position Amount
                "mt":"CROSSED",       // Margin Type
                "iw":"0",             // Isolated Wallet (if isolated position)
                "mp":"9187.17127000", // Mark Price
                "up":"-1.166074",     // Unrealized PnL
                "mm":"1.614445"       // Maintenance Margin Required
              }
            ]
        }  );
        let margin_call: Result<UserStreamEvent, _> = serde_json::from_value(json);
        println!("{:?}", margin_call);
        assert!(margin_call.is_ok());
    }

    #[tokio::test]
    async fn test_userstream_deserialize_account() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/coin_userstream_account.json");
        let fc = std::fs::read_to_string(d).unwrap();

        let account = serde_json::from_str::<UserStreamEvent>(&fc);
        println!("{:?}", account);
        assert!(account.is_ok());
    }

    #[tokio::test]
    async fn test_userstream_deserialize_order_trade() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/coin_userstream_order_trade.json");
        let fc = std::fs::read_to_string(d).unwrap();

        let account = serde_json::from_str::<UserStreamEvent>(&fc);
        println!("{:?}", account);
        assert!(account.is_ok());
    }
}
