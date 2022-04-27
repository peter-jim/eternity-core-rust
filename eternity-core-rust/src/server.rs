use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
use crate::market::*;
use crate::api::*;
use crate::account::*;

#[derive(Debug)]
pub struct Server{
    pub quant_id:String,
    pub threading: JoinHandle<()>,
    pub start_time: String,
    pub account:String,
    pub reciver:Receiver<String>,
    pub sender:Sender<String>,
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


    pub fn AIP_30(num:i32){
       /*
       （Automatic Investment Plan every day,total 30days.
       */

       //step1 初始化，监听来自控制中心的事件

       //step2 执行

       //step3 执行过程写入文件

       //step4 结束运行，向控制中心发送事件。随后线程自毁。

        println!("{:?}",num+num);


    }


    fn crate_server(opt_code:String,api_key: Option<String>, secret_key: Option<String>){
        
    }
   
}

    

