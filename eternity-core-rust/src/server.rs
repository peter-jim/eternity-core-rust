
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
#[derive(Debug)]
pub struct OrderStatus{
    pub clientid:String,// 客户自己设置的ID
    pub price:String,
    pub origqty:String,
    pub status: String,// 用户设置的原始订单数量
    pub types: String,
    pub side: String,
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
        let api_key = Some("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx".into());
        let secret_key = Some("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA".into());
        let account: Account = Binance::new(api_key, secret_key);

        let answer =   account.get_account().unwrap().balances;
      
        let mut usdt:f32 = 0.0;
        
        for i in answer{
            
            if i.asset == "USDT"{
                println!("{:?}",&i.free);
                usdt = i.free.clone().parse().unwrap();
        
            }
        }

        //step2 本地账户初始化  本地状态同步器
        let low_price = 2_f32;

        
        let high_price = 3_f32;
        let grid_num = 20_i32 as f32;


        // let order_status = Vec::new();
        for i in 0..20{
            
                let price = (high_price - low_price)/grid_num + low_price;

        }


        println!(" usdt is {:?}",usdt);
       
        



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

    

