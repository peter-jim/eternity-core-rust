use std::error::Error;
use eternity_core_rust::api::*;
use eternity_core_rust::market::*;
use eternity_core_rust::account::*;
use eternity_core_rust::server::*;
use std::sync::mpsc::channel;
use std::thread;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {

    let  url: String = String::from(API::Spot(Spot::Depth));
    print!("{:?}\n",url);


    let f = File::open("./conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    println!("{:?}", v["name"].as_str().unwrap());
    println!("{:?}", v["binance"]["model"]);

    // let market: Market = Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));
    //   // Latest price for ONE symbol
    //   match market.get_price("BNBUSDT") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }

    // let account:Account =  Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));

    //   match account.get_account() {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }

    //step 0 . 初始化

    let mut stack = Vec::new();

    //step 1. 线程 event-->  监听器监听moonbeam网络事件


    
    
    //step 2. 轮训本机是否有对应服务。

    let web3_event_usdt = String::from("100");
    let web3_event_model = String::from("AIP_30");
    let web3_event_dexaddress = String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx");
    let web3_event_useraddress = String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx");
    let web3_event_serveraddress = String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx");



    if web3_event_model == "AIP_30"{
        
        let (centrial_sender,server_reciver)  = channel();
        let (server_sender,centrial_reciver)  = channel();

        let controler  =  thread::spawn(move||
          Server::AIP_30(server_reciver, server_sender )
        );

        let server =  Server{
          quant_id:stack.len(),
          threading: controler,
          start_time: "2021".to_string(),
          account:"web3_event_useraddress".to_string(),
          
          server_reciver:centrial_reciver,
          centrial_sender:centrial_sender,
          
        };
        println!("stake is {:?}" ,server);
        stack.push(server);
        

        //发送消息
        stack[0].centrial_sender.send(String::from("sxxx"));

        loop {
           //发送消息
             stack[0].centrial_sender.send(String::from("sxxx")); 
             std::thread::sleep(std::time::Duration::from_secs(3));

            

             println!("{:?} 主程序消息   ",stack[0].server_reciver.recv() )

        }
        

        
    }
    else if web3_event_model == "AIP30"{

    }




    for i in v["binance"]["model"].as_array().unwrap(){  
        if &i.as_str().unwrap() == &web3_event_serveraddress{
            println!("{:?}  --  {:?}",&i.as_str().unwrap(),&web3_event_serveraddress);

            // 创建一个对应的服


        }
     
    }

    //step 3.2 启动线程
    


    Ok(())



    // let client = Client::new();
    // let resp = client.get("https://api2.binance.com/api/v3/ticker/price?symbol=BNBUSDT").send()?;
    // println!("Got {:?}", resp);



}


