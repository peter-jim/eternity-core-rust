


#[cfg(test)]
mod tests {
    
    
    #[test]
    fn aip(){
        inital_account() 
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

