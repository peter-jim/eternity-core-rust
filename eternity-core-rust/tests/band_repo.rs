use eternity_core_rust::server::inital_account;
use chrono::prelude::*;
use eternity_core_rust::account;




#[cfg(test)]
mod tests{
    use std::fs::File;

    use chrono::{Local, Timelike, DateTime, Datelike};
    use eternity_core_rust::{market, account::Account, api::Binance};

    use eternity_core_rust::market::*;




    #[test]
    fn band_repo(){
    /*
            轮训[BTC、ETH、DOT、FIL、Link、GLMR]5个系列的代币。判断其是否为下跌10%，15%，20%，25%，如果满足，则下单购买，反弹1-3%立马抛售全部。
            运行最长时间为1个月。1个月后，清算所有资产。
    
    */


   
   



    let now1 = Local::now();


    let now2 = Local::now();

    


    println!("hour now {:?}",now1.hour());
    println!("minute now is {:?}",now1.minute());
    // std::thread::sleep(std::time::Duration::from_secs(10));
    
    println!("now {:?}",now1.max(now2));
    println!("now {:?}",now1.timestamp_millis());
    println!("now {:?}",now1.time().max(now2.time()));
    println!("now {:?}",now1.with_hour(1));

    println!("day is  {:?}",now1.num_days_from_ce());



    is_legal_time(now1);
    



    }


    fn is_legal_time(before:DateTime<Local>)  -> bool{

        let now = Local::now();

        // before.minute();
        // before.hour();
        // before.num_days_from_ce();

        if now.num_days_from_ce() - before.num_days_from_ce() >= 30 {
            if now.hour() >=before.hour(){
                if now.minute() >= before.minute(){
                    println!("合法")
                }
            }
        }
        false

    }


    fn force_sell_all(symbol:String){




    }
    #[test]
    fn aip(){
        // step1 账户初始化
        let account = inital_account();

        // 获取账户余额
        let answer = account.get_account().unwrap();

        //获取交易所订单
        let answer = account.get_open_orders("GLMRBUSD");

        std::thread::sleep(std::time::Duration::from_secs(10));

        println!("{:?}",answer)
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

    pub fn inital_market() ->Market{
        let f = File::open("conf.json").unwrap();
        let v: serde_json::Value = serde_json::from_reader(f).unwrap();
        let api_key = v["binance"]["api_key"].clone().to_string();
        let secret_key = v["binance"]["secret_key"].clone().to_string();
    
        let api_key = Some(api_key.into());
        let secret_key =
            Some(secret_key.into());

        let market: Market = Binance::new(api_key, secret_key);
        market

    }


    #[test]
    fn get_price(){
        let market: Market = inital_market();

        // Latest price for ONE symbol
        match market.get_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    }

}


