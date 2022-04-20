use crate::api::*;
use std::collections::BTreeMap;
use crate::client::*;
use crate::utils::*;
use crate::model::*;
use crate::errors::*;


#[derive(Clone)]
pub struct Market {
    pub client: Client,
    pub recv_window: u64,
}


impl Market{
      // Latest price for ONE symbol.
      pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
      where
          S: Into<String>,
      {
          let mut parameters: BTreeMap<String, String> = BTreeMap::new();
          parameters.insert("symbol".into(), symbol.into());
          let request = build_request(parameters);
          self.client.get(API::Spot(Spot::Price), Some(request))
      }



}