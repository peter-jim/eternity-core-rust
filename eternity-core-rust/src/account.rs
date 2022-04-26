use crate::client::*;
use crate::utils::*;
use crate::api::*;
use crate::model::*;
use crate::errors::*;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}


struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

struct OrderQuoteQuantityRequest {
    pub symbol: String,
    pub quote_order_qty: f64,
    pub price: f64,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}



pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
}

#[allow(clippy::all)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

impl From<OrderType> for String {
    fn from(item: OrderType) -> Self {
        match item {
            OrderType::Limit => String::from("LIMIT"),
            OrderType::Market => String::from("MARKET"),
            OrderType::StopLossLimit => String::from("STOP_LOSS_LIMIT"),
        }
    }
}


pub enum OrderSide {
    Buy,
    Sell,
}

impl From<OrderSide> for String {
    fn from(item: OrderSide) -> Self {
        match item {
            OrderSide::Buy => String::from("BUY"),
            OrderSide::Sell => String::from("SELL"),
        }
    }
}

impl Account {
    pub fn get_account(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Account), Some(request))
    }


}