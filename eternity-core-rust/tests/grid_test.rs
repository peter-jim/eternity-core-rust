#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::server::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
        
    #[test]
    fn grid_test(){
          //step2 本地账户初始化  本地状态同步器
          let low_price = 2.00_f32;
          let high_price = 3.00_f32;
          let grid_num = 20_i32 as f32;
          let mut price = low_price.clone();  
          let mut statusmap = Vec::new();


          
          // let order_status = Vec::new();
          for i in 0..20{
              
                println!("i is {:?}",&i);
                price = (((high_price - low_price)/grid_num + price)*100_f32).round()/100_f32;
                
                println!("pirce is {:?}",price);
                let status = OrderStatus{
                    clientid:"grid_1".to_string(),
                    price:price.clone().to_string(),
                    origqty:"100".to_string(),
                    status: "NEW".to_string(),// 用户设置的原始订单数量
                    types: "LIMIT".to_string(),
                    side: "SELL".to_string()
                };
                statusmap.push(status);            
          }

          // 
          for i in 0..20{
              
                println!("{:?}",statusmap[i])

          }


    }

}