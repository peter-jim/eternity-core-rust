
use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
use crate::market::*;
use crate::api::*;
use crate::account::*;



#[derive(Debug)]
pub struct Server{
    pub quant_id:usize,
    pub threading: JoinHandle<()>,
    pub start_time: String,
    pub account:String,
    pub server_reciver:Receiver<String>,
    pub centrial_sender:Sender<String>,
}
#[derive(Debug,Clone)]
pub struct OrderStatus{
    pub clientid:String,// 客户自己设置的ID
    pub price:String,
    pub origqty:String,
    pub status: String,// 用户设置的原始订单数量
    pub types: String,
    pub side: String,
    pub  compare: String,
}
pub fn inital_account() -> Account{
    let api_key = Some("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx".into());
    let secret_key = Some("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA".into());
    let account: Account = Binance::new(api_key, secret_key);
    account
}

impl Server {

   
    
    
    pub fn get_price(){
        println!("hello");
        let market: Market = Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));
        // Latest price for ONE symbol
        match market.get_price("BNBUSDT") {
          Ok(answer) => println!("{:?}", answer),
          Err(e) => println!("Error: {}", e),
      }
    }

    pub fn get_account(){
        let account:Account =  Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));

         match account.get_account() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
     }
    }


    pub fn AIP_30(server_reciver:Receiver<String>, server_sender:Sender<String>){
       /*
       （Automatic Investment Plan every day,total 30days.
       */
       
       //step1 初始化，监听来自控制中心的事件

       //step2 执行

       //step3 执行过程写入文件

       //step4 结束运行，向控制中心发送事件。随后线程自毁。

        println!("{:?}","服务已启动");

        // step1 账户初始化
        let account = inital_account();


        // 获取账户余额
        let answer =   account.get_account().unwrap().balances;

        //获取交易所订单
        let answer  = account.get_open_orders("GLMRBUSD");

        //获取市价
        let market: Market = Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));
        let result_price = market.get_price("GLMRBUSD");

       
        if result_price.is_ok(){
            let  now_price = result_price.unwrap().price;

            println!(" 现在GLMR BUSD 的市价是 {:?} ",now_price);
        }
        

        let mut orderlist = Vec::new();
        if answer.is_ok(){
            for i in answer.unwrap(){
                //println!("{:?}",&i   );
                // let writer = BufWriter::new(File::create("./log/order.json").unwrap());
                // serde_json::to_writer_pretty(writer, &i).unwrap();
                
                let  serder = OrderStatus{
                clientid:i.clone().client_order_id .to_string() ,
                price:i.clone().price.to_string(),
                origqty:i.clone().orig_qty.to_string(),
                status: i.clone().status.to_string(),// 用户设置的原始订单数量
                types: i.clone().type_name.to_string(),
                side: i.clone().side.to_string(),
                compare: "0".to_string(),
                };
                orderlist.push(serder);
            }   
             
            // for i in orderlist{
            //     println!("{:?}",&i);
            // }  
        }

        let  usdt:f32 = 0.0;  
        //step2 本地账户初始化  本地状态同步器
        let low_price = 2_f32;
        let high_price = 3_f32;
        let grid_num = 20_i32 as f32;


        let low_price = 2.00_f32;
        let high_price = 3.00_f32;
        let grid_num = 20_i32 as f32;
        let mut price = low_price.clone();  
        let mut statusmap = Vec::new();
        

        
        // let order_status = Vec::new();
        for i in 0..20{
            
            //   println!("i is {:?}",&i);
              price = (((high_price - low_price)/grid_num + price)*100_f32).round()/100_f32;
              
            //   println!("pirce is {:?}",price);
              let status = OrderStatus{
                  clientid:"grid_".to_string() ,
                  price:price.clone().to_string(),
                  origqty:"100".to_string(),
                  status: "NEW".to_string(),// 用户设置的原始订单数量
                  types: "LIMIT".to_string(),
                  side: "SELL".to_string(),
                  compare:"0".to_string()
              };
              statusmap.push(status);            
        }

        for i in 0..20{
              if statusmap[i].price.parse::<f32>().unwrap() > 2.50{
                  statusmap[i].side = "BUY".to_string();
              }
              // println!("{:?}",statusmap[i].price.parse::<f32>().unwrap());
        }

        for i in 0..20{
          if statusmap[i].side != statusmap[i+1].side{
              statusmap[i].side = "None".to_string();
              break;
          }
       }

        // 
        for i in 0..20{
              println!("{:?}",statusmap[i])
        }
       
        
        //网格 订单系统
  
        for i in 0..20{
            for j in orderlist.clone(){
                // step 1 如果 orderlist 没有 ，则代代表需要向交易所发送相关指令，如果有则不用更新。
                   
                if statusmap[i].clientid == j.clientid{
                     statusmap[i].compare="1".to_string();  //1 代表交易所需要更新指令。
                }
            }
        }
        
        // 向交易所发送 现价订单

        for i in 0..20{

            println!(" now price is {:?}",now_price);
            
            if statusmap[i].price.parse::<f32>().unwrap()  < now_price{
                statusmap[i].side = "BUY".to_string()
               }else {
                statusmap[i].side = "SELL".to_string()
               }
               println!(" grid price is {:?}",statusmap[i].price.parse::<f32>().unwrap());
            println!("{:?}",statusmap[i]);
        }






        loop {
            
            let market: Market = Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));
            // Latest price for ONE symbol
            match market.get_price("BNBUSDT") {
                Ok(answer) => println!("{:?}", answer),
                Err(e) => println!("Error: {}", e),
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
            println!(" 接收到消息 {:?} ",server_reciver.recv());
            server_sender.send(String::from("hello center")).unwrap();
        }


        


    }


    fn crate_server(opt_code:String,api_key: Option<String>, secret_key: Option<String>){
        
    }
   
}

    

