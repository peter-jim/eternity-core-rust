
use eternity_core_rust::account;

#[cfg(test)]
mod tests {
    
    
    #[test]
    fn aip(){
        let account = inital_account() ;
        let result_price = market.get_price("GLMRBUSD");
    }
    
    
    pub fn inital_account() -> Account {
    
        let f = File::open("conf.json").unwrap();
        let v: serde_json::Value = serde_json::from_reader(f).unwrap();
        let api_key = v["binance"]["api_key"].clone().to_string();
        let secret_key = v["binance"]["secret_key"].clone().to_string();
    
        let api_key = Some(api_key.into());
        let secret_key =
            Some(secret_key.into());
        let account: Account = Binance::new(api_key, secret_key);
        account
    }


    
}

