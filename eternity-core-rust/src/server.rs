use serde::Serialize;
use crate::account::*;
use crate::api::*;
use crate::event::Event;
use crate::market::*;
use crate::mpscanaly::*;
use crate::mysql::update_event_pending;
use crate::mysql::update_event_runing;
use crate::mysql::update_option_running;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;
use std::{sync::mpsc::Receiver, sync::mpsc::Sender, thread::JoinHandle};
use secp256k1::SecretKey;
use web3::{
    contract::{Contract, Options},
};
use std::fs::File;
use std::io::BufWriter;



#[derive(Debug)]
pub struct Server {
    pub threading: JoinHandle<()>,
    pub server_reciver: Receiver<OptionCode>,
    pub centrial_sender: Sender<OptionCode>,

    pub balance: f32,
    pub dexaddress: String,
    pub model: String,
    pub serveraddress: String,
    pub transactionhash: String,
    pub useraddress: String,
}
#[derive(Debug, Clone)]
pub struct OrderStatus {
    pub clientid: String, // 客户自己设置的ID
    pub price: String,
    pub origqty: String,
    pub status: String, // 用户设置的原始订单数量
    pub types: String,
    pub side: String,
    pub compare: String,
}

pub fn inital_account() -> Account {
    let api_key = Some("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx".into());
    let secret_key =
        Some("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA".into());
    let account: Account = Binance::new(api_key, secret_key);
    account
}

pub fn post_ser() {}

impl Server {
    pub fn get_price() {
        println!("hello");
        let market: Market = Binance::new(
            Option::Some(String::from(
                "y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx",
            )),
            Option::Some(String::from(
                "GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA",
            )),
        );
        // Latest price for ONE symbol
        match market.get_price("BNBUSDT") {
            Ok(answer) => println!("{:?}", answer),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn get_account() {
        let account: Account = Binance::new(
            Option::Some(String::from(
                "y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx",
            )),
            Option::Some(String::from(
                "GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA",
            )),
        );

        match account.get_account() {
            Ok(answer) => println!("{:?}", answer),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn AIP(server_reciver: Receiver<OptionCode>, server_sender: Sender<OptionCode>,event:Event) {
        println!("启动线程 AIP");
        let e = event;

        loop {
            
            // 1. 创建对应的量化程序
            create_aip();
            // 2. 接收来自main的消息
            let result = recv_main(&server_reciver,e.clone());
           
            // 3. 发送线程信息到中性化服务器 
            send_info(100,e.clone());

            if result == true{
                println!("服务完毕结束线程");
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(3));

            
        }
    }

    pub fn AIP_30(server_reciver: Receiver<OptionCode>, server_sender: Sender<OptionCode>) {
        /*
        （Automatic Investment Plan every day,total 30days.
        */

        //step1 初始化，监听来自控制中心的事件

        //step2 执行

        //step3 执行过程写入文件

        //step4 结束运行，向控制中心发送事件。随后线程自毁。

        println!("{:?}", "服务已启动");

        // step1 账户初始化
        let account = inital_account();

        // 获取账户余额
        let answer = account.get_account().unwrap().balances;

        //获取交易所订单
        let answer = account.get_open_orders("GLMRBUSD");

        loop {
            // step1 账户初始化
            let account = inital_account();

            // 获取账户余额
            let answer = account.get_account().unwrap().balances;

            //获取交易所订单
            let answer = account.get_open_orders("GLMRBUSD");
            //获取市价
            let market: Market = Binance::new(
                Option::Some(String::from(
                    "y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx",
                )),
                Option::Some(String::from(
                    "GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA",
                )),
            );
            let result_price = market.get_price("GLMRBUSD");
            let mut now_price = 0.0;
            if result_price.is_ok() {
                now_price = result_price.unwrap().price;

                println!(" 现在GLMR BUSD 的市价是 {:?} ", now_price);
            }

            let mut orderlist = Vec::new();

            if answer.is_ok() {
                for i in answer.unwrap() {
                    //println!("{:?}",&i   );
                    // let writer = BufWriter::new(File::create("./log/order.json").unwrap());
                    // serde_json::to_writer_pretty(writer, &i).unwrap();

                    let serder = OrderStatus {
                        clientid: i.clone().client_order_id.to_string(),
                        price: i.clone().price.to_string(),
                        origqty: i.clone().orig_qty.to_string(),
                        status: i.clone().status.to_string(), // 用户设置的原始订单数量
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

            let usdt: f32 = 0.0;
            //step2 本地账户初始化  本地状态同步器
            let low_price = 2_f32;
            let high_price = 3_f32;
            let grid_num = 20_i32 as f32;

            let low_price = 1.700_f32;
            let high_price = 2.50_f32;
            let grid_num = 20_i32 as f32;
            let mut price = low_price.clone();
            let mut statusmap = Vec::new();

            // let order_status = Vec::new();
            for i in 0..20 {
                //   println!("i is {:?}",&i);
                price = (((high_price - low_price) / grid_num + price) * 100_f32).round() / 100_f32;

                //   println!("pirce is {:?}",price);
                let status = OrderStatus {
                    clientid: "grid_".to_string() + &i.clone().to_string(),
                    price: price.clone().to_string(),
                    origqty: "10".to_string(),
                    status: "NEW".to_string(), // 用户设置的原始订单数量
                    types: "LIMIT".to_string(),
                    side: "SELL".to_string(),
                    compare: "0".to_string(),
                };
                statusmap.push(status);
            }

            for i in 0..20 {
                if statusmap[i].price.parse::<f32>().unwrap() > 2.50 {
                    statusmap[i].side = "BUY".to_string();
                }
                // println!("{:?}",statusmap[i].price.parse::<f32>().unwrap());
            }

            //网格 订单系统

            for i in 0..20 {
                for j in orderlist.clone() {
                    // step 1 如果 orderlist 没有 ，则代代表需要向交易所发送相关指令，如果有则不用更新。

                    if statusmap[i].clientid == j.clientid {
                        statusmap[i].compare = "1".to_string(); //1 代表交易所需要更新指令。
                    }
                }
            }

            // 向交易所发送 现价订单
            for i in 0..20 {
                // println!(" now price is {:?}",now_price);
                if statusmap[i].price.parse::<f64>().unwrap() < now_price {
                    statusmap[i].side = "BUY".to_string()
                } else {
                    statusmap[i].side = "SELL".to_string()
                }
                //    println!(" grid price is {:?}",statusmap[i].price.parse::<f32>().unwrap());
                println!("{:?}", statusmap[i]);
            }

            for i in 0..20 {
                if statusmap[i].side != statusmap[i + 1].side {
                    statusmap[i - 1].side = "None".to_string();
                    break;
                }
            }

            //
            for i in 0..20 {
                if now_price <= statusmap[0].price.parse::<f64>().unwrap() || now_price >= 2.50 {
                    break;
                }

                println!(" 开始下单 ");

                if statusmap[i].compare == "0" {
                    if statusmap[i].side == "BUY" {
                        let result = account
                            .limit_buy(
                                "GLMRBUSD",
                                statusmap[i].origqty.parse::<f64>().unwrap(),
                                statusmap[i].price.parse::<f64>().unwrap(),
                                statusmap[i].clientid.to_string(),
                            )
                            .unwrap();
                        println!(" result is {:?} ", result);
                    }
                }
                println!("{:?}", statusmap[i])
            }

            std::thread::sleep(std::time::Duration::from_secs(3));

            //对消息启动解析器
            server_sender.send(OptionCode::AllBalance).unwrap();
            println!(" 接收到消息 {:?} ", server_reciver.recv());
        }
    }

    pub fn grid_glmr_20(server_reciver: Receiver<OptionCode>, server_sender: Sender<OptionCode>) {
        let optioncode = OptionCode::Shoutdown;
        let rev = server_reciver.recv().unwrap();
        match rev {
            OptionCode::Shoutdown => {
                optioncode.get_open_orders();
                //取消所有的订单

                //检查所有的订单是否取消

                //如果因为网络问题，没有取消则继续取消。

                //3次发送 网络错误
            }
            OptionCode::AllBalance => {
                println!("xxx");

                //获取账户余额

                //通过send发送回去
            }
            OptionCode::AllOrder => {
                println!("xxx")
                //获取账户订单

                //通过send发送回去
            }
            OptionCode::ErrorStatus => {
                println!("xxx")
                //返回ErrorStatus列表
            }

            OptionCode::Withdraw => {
                println!("xxx")
                //返回ErrorStatus列表
            }
        }
    }
}



fn create_aip(){

}

fn recv_main(server_reciver: &Receiver<OptionCode>,event:Event)-> bool{
    let rev = server_reciver.recv();

    match rev {
        Ok(_) => {
            println!("成功接收主线程");


            match rev.unwrap() {
                OptionCode::Shoutdown => {
                    return true;
                    //取消所有的订单
        
                    //检查所有的订单是否取消
        
                    //如果因为网络问题，没有取消则继续取消。
        
                    //3次发送 网络错误
                }
                OptionCode::AllBalance => {
                    println!("xxx");
                    return true;
        
                    //获取账户余额
        
                    //通过send发送回去
                }
                OptionCode::AllOrder => {
                    println!("xxx") ;
                    return true;
                    //获取账户订单
        
                    //通过send发送回去
                }
                OptionCode::ErrorStatus => {
                    println!("xxx");
                    //返回ErrorStatus列表
                    return true;
                }
        
                OptionCode::Withdraw => {
                    println!("启动提款线程");
                    let result =  send_event_to_moonbeam();
                    //返回ErrorStatus列表
                     //step 1，检查返回状态
                    //step 2. 关闭服务。
                    // update(event);
                    update_event_runing(event.transactionhash.clone());
                    update_option_running(event.transactionhash.clone());

                    
                    return true;
        
                }
                
            }
        

        },
        Err(_) => {
            println!("接收主线程出错")
        },
    }


    return false; 
}

//send info to server node
fn send_info( usdt:i32,event:Event){
    
    #[derive(Serialize)]
        struct Info<'a> {
            profile: f32,
            balance:f32,
            dexaddress:&'a str,
            model:&'a str,
            serveraddress:&'a str,
            transactionhash:&'a str,
            useraddress:&'a str,           
        }

    println!(" 更新信息到node  ");
    let client = reqwest::blocking::Client::builder()
    .pool_idle_timeout(None)
    .build()
    .unwrap();

    let response = client.get("http://127.0.0.1:5000/node").json(&Info{
        profile: 100.0,
        balance:event.balance,
        dexaddress:&event.dexaddress,
        model:&event.model,
        serveraddress:&event.serveraddress,
        transactionhash:&event.transactionhash,
        useraddress:&event.useraddress,  
    }).send().ok();


}





#[tokio::main]
async fn send_event_to_moonbeam() -> web3::contract::Result<()> {
    println!("start run web3: ");
    let transport = web3::transports::Http::new("https://rpc.testnet.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("d028d24f16a8893bd078259d413372ac01580769".parse().unwrap());

    let private_key =
        SecretKey::from_str("e7688610e0ebfccbac5c9c5d637db2910d4b64f6f36460de6b964f4c725c9f95")
            .unwrap();

    let contract = Contract::from_json(
        web3.eth(),
        "d028d24f16a8893bd078259d413372ac01580769".parse().unwrap(),
        include_bytes!("../res/demo.abi"),
    )
    .unwrap();

    // let tx =  contract.signed_call("putOrder", (accounts[0],1_000_000_u64), Options::default(), & private_key).await?;

    // println!("tx is {:?}",&tx);
    // let tx = contract.call("putOrder", (accounts[0],1_000_000_u64), accounts[0], Options::default()).await?;
    // println!("got tx: {:?}", tx);

    let tx_re = contract
        .signed_call_with_confirmations(
            "putOrder",
            (accounts[0], 1_000_000_u64),
            Options::default(),
            1,
            &private_key,
        )
        .await?;

        
    println!("确认后的交易是 {:?}", &tx_re);

    std::thread::sleep(std::time::Duration::from_secs(30));

    Ok(())
}

//更新op_runing ---> op_finish   runing ---> finish
fn update(event:Event){
    let f_op_running = File::open("./storage/op_running.json").unwrap();
    let f_op_finish = File::open("./storage/op_finish.json").unwrap();
    let f_running = File::open("./storage/running.json").unwrap();
    let f_finish = File::open("./storage/finish.json").unwrap();

    let v_op_running: serde_json::Value = serde_json::from_reader(f_op_running).unwrap();
    let v_op_finish: serde_json::Value = serde_json::from_reader(f_op_finish).unwrap();
    let v_running: serde_json::Value = serde_json::from_reader(f_running).unwrap();
    let v_finish: serde_json::Value = serde_json::from_reader(f_finish).unwrap();

    let mut arrary_op_running = v_op_running.as_array().unwrap().clone();
    let mut arrary_op_finish = v_op_finish.as_array().unwrap().clone();
    let mut arrary_running = v_running.as_array().unwrap().clone();
    let mut arrary_finish = v_finish.as_array().unwrap().clone();


    //remove op_runing ,and add to finish
    for i in 0..arrary_op_running.len(){
        if arrary_op_running.get(i).unwrap()["transactionHash"] == event.transactionhash{
           


            arrary_op_finish.push( arrary_op_running[i].clone());
            arrary_op_running.remove(i);

            let writer = BufWriter::new(File::create("./storage/op_finish.json").unwrap());
            serde_json::to_writer_pretty(writer, &arrary_running).unwrap();
        }

    }

    //remove runing,and add to finish
    for i in 0..arrary_running.len(){
        if arrary_running.get(i).unwrap()["transactionHash"] == event.transactionhash{
            arrary_finish.push( arrary_running[i].clone());
            arrary_running.remove(i);

            let writer = BufWriter::new(File::create("./storage/finish.json").unwrap());
            serde_json::to_writer_pretty(writer, &arrary_finish).unwrap();


        }
    }

}




pub fn create_server(event:Event) -> Result<Server,String>{

    let (centrial_sender, server_reciver) = channel();
    let (server_sender, centrial_reciver) = channel();

    let e = event.clone();
 
    match event.model.as_str() {
        "AIP" => {
            println!("创建服务");
            let controler =
            thread::spawn(move || Server::AIP(server_reciver, server_sender, event.clone()));
            let server = build_server(e, centrial_sender, centrial_reciver, controler);
            return Result::Ok(server)
        }

        //如果我们有新的程序更新，再这添加即可。

        _ =>{
            println!("create error");
            return Result::Err("create error".to_string());
        }
    }

}



fn build_server(
    event: Event,
    centrial_sender: Sender<OptionCode>,
    centrial_reciver: Receiver<OptionCode>,
    controler: thread::JoinHandle<()>,
) -> Server{

    let server = Server {
        threading: controler,
        server_reciver: centrial_reciver,
        centrial_sender: centrial_sender,
        balance: event.balance,
        dexaddress: event.dexaddress,
        model: event.model,
        serveraddress: event.serveraddress,
        transactionhash: event.transactionhash.clone(),
        useraddress: event.useraddress,
    };

    //更新到数据库
    update_event_pending(event.transactionhash);

    return server
}
