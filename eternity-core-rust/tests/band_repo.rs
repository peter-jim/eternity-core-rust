use eternity_core_rust::server::inital_account;
use chrono::prelude::*;




#[cfg(test)]
mod tests{
    use chrono::{Local, Timelike, DateTime, Datelike};






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


}


