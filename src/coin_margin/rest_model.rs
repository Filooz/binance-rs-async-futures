pub use crate::rest_model::{string_or_float, string_or_float_opt, string_or_u64, Asks, Bids, BookTickers,
                            KlineSummaries, KlineSummary, OrderSide, OrderStatus, RateLimit, ServerTime, SymbolPrice,
                            SymbolStatus, Tickers, TimeInForce};
use crate::{futures::rest_model::{Filters, PositionSide},
            rest_model::OrderType};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub exchange_filters: Vec<Filters>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: u64,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
    // does not exist in dapi (26.3.24)
    // pub assets: Vec<AssetDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub filters: Vec<Filters>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
    #[serde(with = "string_or_float")]
    pub liquidation_fee: f64,
    #[serde(with = "string_or_float")]
    pub market_take_bound: f64,
    pub symbol: String,
    pub pair: String,
    pub contract_type: ContractType,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub contract_status: ContractStatus,
    pub contract_size: u64,
    pub quote_asset: String,
    pub base_asset: String,
    pub margin_asset: String,
    pub price_precision: u64,
    pub quantity_precision: u64,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub equal_qty_precision: u64,
    #[serde(with = "string_or_float")]
    pub trigger_protect: f64,
    #[serde(with = "string_or_float", rename = "maintMarginPercent")]
    pub maintenance_margin_percent: f64,
    #[serde(with = "string_or_float")]
    pub required_margin_percent: f64,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    #[serde(rename = "NEXT_QUARTER DELIVERING")]
    NextQuarterDelivery,
    #[serde(rename = "CURRENT_QUARTER DELIVERING")]
    CurrentQuarterDelivery,
    #[serde(rename = "PERPETUAL DELIVERING")]
    PerpetualDelivery,
    #[serde(rename = "")]
    Empty,
}

use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndex {
    pub symbol: String,
    pub pair: Option<String>,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub index_price: f64,
    #[serde(with = "string_or_float")]
    pub estimated_settle_price: f64,
    #[serde(with = "string_or_float_opt")]
    pub last_funding_rate: Option<f64>,
    #[serde(with = "string_or_float_opt")]
    pub interest_rate: Option<f64>,
    pub next_funding_time: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PremiumIndexResponse {
    One(PremiumIndex),
    Many(Vec<PremiumIndex>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prem_index() {
        let res = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "markPrice": "11029.69574559",
                "indexPrice": "10979.14437500",
                "estimatedSettlePrice": "10981.74168236",
                "lastFundingRate": "0.00071003",
                "interestRate": "0.00010000",
                "nextFundingTime": 1596096000000,
                "time": 1596094042000
            },
            {
                "symbol": "BTCUSD_200925",  
                "pair": "BTCUSD",
                "markPrice": "12077.01343750",
                "indexPrice": "10979.10312500",
                "estimatedSettlePrice": "10981.74168236",
                "lastFundingRate": "",
                "interestRate": "", 
                "nextFundingTime": 0,
                "time": 1596094042000
            },
            {
                "symbol": "BTCUSD_PERP",
                "markPrice": "11029.69574559",
                "indexPrice": "10979.14437500",
                "estimatedSettlePrice": "10981.74168236",
                "lastFundingRate": "0.00071003",
                "interestRate": "0.00010000",
                "nextFundingTime": 1596096000000,
                "time": 1596094042000
            }
        ]"#;

        let deser = serde_json::from_str::<Vec<PremiumIndex>>(res);
        println!("{:?}", deser);
        assert!(deser.is_ok());
    }
}
