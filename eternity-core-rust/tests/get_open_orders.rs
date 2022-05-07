#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::market::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
    use eternity_core_rust::server::*;
    
    use std::io::BufWriter;
    use std::io::BufReader;
    use std::fs::File;
        
    #[test]
    fn get_open_orders_test(){
        let api_key = Some("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx".into());
        let secret_key = Some("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA".into());
    
        let account: Account = Binance::new(api_key, secret_key);
        // match account.market_buy("GLMRBUSD", 5) {
        // Ok(answer) => {

        //     println!(" result is   {:?}", &answer);
        

            
        //     println!("{:?}", answer)
        // }
        

        //获取市场当前的挂单
        match account.get_open_orders("GLMRBUSD") {
            Ok(answer) => 
            {
                let writer = BufWriter::new(File::create("./log/order.json").unwrap());
                serde_json::to_writer_pretty(writer, &answer).unwrap();
                
                let mut orderlist = Vec::new();

                for i in answer{
                    //println!("{:?}",&i   );
                    // let writer = BufWriter::new(File::create("./log/order.json").unwrap());
                    // serde_json::to_writer_pretty(writer, &i).unwrap();
                    let mut serder = OrderStatus{
                    clientid:i.clone().client_order_id .to_string() ,
                    price:i.clone().price.to_string(),
                    origqty:i.clone().orig_qty.to_string(),
                    status: i.clone().status.to_string(),// 用户设置的原始订单数量
                    types: i.clone().type_name.to_string(),
                    side: i.clone().side.to_string()
                    };
                    orderlist.push(serder);

                }

                for i in orderlist{
                    println!("{:?}",&i);
                }          
               
            }
            
            
            Err(e) => println!("Error: {}", e),
        }
        
        //[Order { symbol: "GLMRBUSD", order_id: 66058175, order_list_id: -1, client_order_id: "and_9f49bc27f75e400388991b1301647da5", price: 3.0, orig_qty: "5.00000000", executed_qty: "0.00000000", cummulative_quote_qty: "0.00000000", status: "NEW", time_in_force: "GTC", type_name: "LIMIT", side: "SELL", stop_price: 0.0, iceberg_qty: "0.00000000", time: 1651650893818, update_time: 1651650893818, is_working: true, orig_quote_order_qty: "0.00000000" }, Order { symbol: "GLMRBUSD", order_id: 66058190, order_list_id: -1, client_order_id: "and_24f54239e8f04c0d881dc5d4fcf17394", price: 4.0, orig_qty: "5.00000000", executed_qty: "0.00000000", cummulative_quote_qty: "0.00000000", status: "NEW", time_in_force: "GTC", type_name: "LIMIT", side: "SELL", stop_price: 0.0, iceberg_qty: "0.00000000", time: 1651650898320, update_time: 1651650898320, is_working: true, orig_quote_order_qty: "0.00000000" }, Order { symbol: "GLMRBUSD", order_id: 66058500, order_list_id: -1, client_order_id: "and_136ada279d234ad6a5d4fc0be8b3939d", price: 1.2, orig_qty: "10.00000000", executed_qty: "0.00000000", cummulative_quote_qty: "0.00000000", status: "NEW", time_in_force: "GTC", type_name: "LIMIT", side: "BUY", stop_price: 0.0, iceberg_qty: "0.00000000", time: 1651650980343, update_time: 1651650980343, is_working: true, orig_quote_order_qty: "0.00000000" }]

    }

}