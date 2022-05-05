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
        
        //{ symbol: "GLMRBUSD", order_id: 65915308, order_list_id: Some(-1), client_order_id: "fsc1JVljEIDXgh2KqVcK72", transact_time: 1651586177214, price: 0.0, orig_qty: 5.0, executed_qty: 5.0, cummulative_quote_qty: 12.5075, stop_price: 0.0, status: "FILLED", time_in_force: "GTC", type_name: "MARKET", side: "BUY", fills: Some([FillInfo { price: 2.5015, qty: 5.0, commission: 0.005, commission_asset: "GLMR", trade_id: Some(2741749) }]) } 
        


    }

}