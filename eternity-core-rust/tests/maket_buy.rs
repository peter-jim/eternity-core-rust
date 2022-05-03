#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::market::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
        
    #[test]
    fn market_buy_test(){
        let api_key = Some("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx".into());
        let secret_key = Some("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA".into());
    
        let account: Account = Binance::new(api_key, secret_key);
        // match account.market_buy("GLMRBUSD", 5) {
        // Ok(answer) => {

        //     println!(" result is   {:?}", &answer);
        

            
        //     println!("{:?}", answer)
        // }
        
        let result = account.market_buy("GLMRBUSD", 5).unwrap();
        println!(" result is xxxx");
        println!(" result is {:?} " , result);
        
        


    }

}