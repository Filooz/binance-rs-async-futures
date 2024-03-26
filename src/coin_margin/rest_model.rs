pub use crate::rest_model::{string_or_float, string_or_u64, Asks, Bids, BookTickers, KlineSummaries, KlineSummary,
                            OrderSide, OrderStatus, RateLimit, ServerTime, SymbolPrice, SymbolStatus, Tickers,
                            TimeInForce};
use crate::{futures::rest_model::PositionSide, rest_model::OrderType};

use serde::{Deserialize, Serialize};

fn default_stop_price() -> f64 { 0.0 }
fn default_activation_price() -> f64 { 0.0 }
fn default_price_rate() -> f64 { 0.0 }

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_base: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: OrderSide,
    pub reduce_only: bool,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub orig_type: OrderType,
    #[serde(with = "string_or_float", default = "default_activation_price")]
    pub activate_price: f64,
    #[serde(with = "string_or_float", default = "default_price_rate")]
    pub price_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub fee_tier: u64,
    pub update_time: u64,
    pub assets: Vec<AccountAsset>,
    pub positions: Vec<AccountPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountAsset {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountPosition {
    pub symbol: String,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float", rename = "maintMargin")]
    pub maintenance_margin: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    #[serde(with = "string_or_u64")]
    pub leverage: u64,
    pub isolated: bool,
    pub position_side: PositionSide,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub break_even_price: f64,
    #[serde(with = "string_or_float")]
    pub max_qty: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub balance: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_available: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    pub update_time: u64,
}
