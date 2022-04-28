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


    //step 1. 监听器监听moonbeam网络事件

    //step 2. 轮训本机是否有对应服务。

    let event_info = "AIP";
    //step 3. 启动对应服务的线程。

    //step 3.1 传入币安子账户 

    //step 3.2 启动线程

    let mut stack = Vec::new();
    

    for i in 1..10 {
      
      let (tx1,rx1)  = channel();
      let (tx,rx)  = channel();

      let controler  =  thread::spawn(move||
        Server::AIP_30(i.clone())
      );

      let server =  Server{
        quant_id:i.clone().to_string(),
        threading: controler,
        start_time: "2021".to_string(),
        account:"账户1".to_string(),
        reciver:rx1,
        sender:tx,
      };

      println!("stake is {:?}" ,server);
      stack.push(server);
      // println!("stake is {:?}" ,stack[0]);

      println!("xxx stake is {:?}" ,stack[0]);

   }

   println!("{:?}",111);
    Ok(())



    // let client = Client::new();
    // let resp = client.get("https://api2.binance.com/api/v3/ticker/price?symbol=BNBUSDT").send()?;
    // println!("Got {:?}", resp);



}
