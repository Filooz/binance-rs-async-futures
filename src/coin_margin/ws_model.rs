use crate::futures::rest_model::OrderType;
use crate::futures::rest_model::PositionSide;
use rust_decimal::Decimal;
use serde;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::primitive::str;

use super::rest_model::OrderSide;
use super::rest_model::OrderStatus;
use super::rest_model::TimeInForce;
pub use crate::rest_model::{string_or_float, string_or_float_opt};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "e")]
pub enum UserStreamEvent {
    MarginCall(MarginCallEvent),
    AccountUpdate(AccountUpdateEvent),
    OrderTradeUpdate(OrderTradeUpdateEvent),
}

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

    #[serde(default, rename = "n", with = "string_or_float_opt")]
    pub commission: Option<f64>,

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

    #[serde(default, rename = "n", with = "string_or_float_opt")]
    pub activation_price: Option<f64>,

    #[serde(default, rename = "cr", with = "string_or_float_opt")]
    pub callback_rate: Option<f64>,

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

    use serde_json::Value;

    use crate::coin_margin::ws_model::UserStreamEvent;

    #[test]
    fn test_deserialize_userstream() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/coin_userstream.json");
        let fc = std::fs::read_to_string(d).unwrap();

        let val_vec = serde_json::from_str::<Vec<Value>>(&fc).unwrap();
        for val in val_vec {
            match serde_json::from_value::<UserStreamEvent>(val.clone()) {
                Ok(v) => {}
                Err(e) => {
                    println!("{:?}", val);
                    panic!("{:?}", e)
                }
            }
        }
    }
}
