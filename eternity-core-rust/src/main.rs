use eternity_core_rust::account::*;
use eternity_core_rust::api::*;
use eternity_core_rust::market::*;
use eternity_core_rust::mpscanaly::*;
use eternity_core_rust::server::*;
use eternity_core_rust::event::*;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::channel;
use std::thread;


fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("./conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    println!("{:?}", v["name"].as_str().unwrap());
    println!("{:?}", v["binance"]["model"]);

    //step 0 . 初始化
    let mut stack = Vec::new();

    //step 1. 线程 event-->  监听器监听moonbeam网络事件

    //step 2. 轮训本机是否有对应服务。

    let web3_event_usdt = String::from("100");
    let web3_event_model = String::from("AIP_30");
    let web3_event_dexaddress =
        String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx");
    let web3_event_useraddress =
        String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx");
    let web3_event_serveraddress =
        String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx");

    if web3_event_model == "AIP_30" {
        let (centrial_sender, server_reciver) = channel();
        let (server_sender, centrial_reciver) = channel();

        let controler = thread::spawn(move||
          // Server::AIP_30(server_reciver, server_sender )
          Server::AIP_30(server_reciver, server_sender));

        let server = Server {
            quant_id: stack.len(),
            threading: controler,
            start_time: "2021".to_string(),
            account: "web3_event_useraddress".to_string(),

            server_reciver: centrial_reciver,
            centrial_sender: centrial_sender,
        };
        println!("stake is {:?}", server);
        stack.push(server);

        //发送消息
        stack[0].centrial_sender.send(OptionCode::Shoutdown);

        loop {
            //发送消息
            stack[0].centrial_sender.send(OptionCode::ErrorStatus);
            std::thread::sleep(std::time::Duration::from_secs(3));

            println!("{:?} 主程序消息   ", stack[0].server_reciver.recv())
        }
    } else if web3_event_model == "AIP30" {
    }

    for i in v["binance"]["model"].as_array().unwrap() {
        if &i.as_str().unwrap() == &web3_event_serveraddress {
            println!(
                "{:?}  --  {:?}",
                &i.as_str().unwrap(),
                &web3_event_serveraddress
            );

            // 创建一个对应的服
        }
    }

    //step 3.2 启动线程

    Ok(())
}

fn updata_conf() {
    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let serveraddress = v["binance"]["serveraddress"].clone();
    updata_event_by_station(serveraddress);
}

fn updata_event_by_station(serveraddress: Value) {
    let f_pending = File::open("./storage/pending.json").unwrap();
    let f_finish = File::open("./storage/finish.json").unwrap();
    let f_running = File::open("./storage/running.json").unwrap();

    let v_pending: serde_json::Value = serde_json::from_reader(f_pending).unwrap();
    let v_finish: serde_json::Value = serde_json::from_reader(f_finish).unwrap();
    let v_running: serde_json::Value = serde_json::from_reader(f_running).unwrap();

    let mut arrary_pending = v_pending.as_array().unwrap().clone();
    let mut arrary_finish = v_finish.as_array().unwrap().clone();
    let mut arrary_running = v_running.as_array().unwrap().clone();

    #[derive(Serialize)]
    struct Event<'a> {
        nodeaddress: &'a str,
        body: &'a str,
    }

    let client = reqwest::blocking::Client::builder()
        .pool_idle_timeout(None)
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:5000/chaindata")
        .json(&Event {
            nodeaddress: serveraddress.as_str().unwrap(),
            body: "json",
        })
        .send()
        .ok();

    let mut array = response
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap()
        .as_array()
        .unwrap()
        .clone();

    //array must not in finish , pending anding running. If
    for i in 0..array.clone().len() {
        for j in 0..arrary_finish.len() {
            if array.get(i).unwrap()["tracnsactionHash"]
                == arrary_finish.get(j).unwrap()["tracnsactionHash"]
            {
                array.remove(i);
            }
        }
    }

    // not in runing
    for i in 0..array.clone().len() {
        for j in 0..arrary_running.len() {
            if array.get(i).unwrap()["tracnsactionHash"]
                == arrary_running.get(j).unwrap()["tracnsactionHash"]
            {
                array.remove(i);
            }
        }
    }

    //not in pending
    for i in 0..array.clone().len() {
        for j in 0..arrary_pending.len() {
            if array.get(i).unwrap()["tracnsactionHash"]
                == arrary_pending.get(j).unwrap()["tracnsactionHash"]
            {
                array.remove(i);
            }
        }
    }

    //add array to pending
    arrary_pending.append(&mut array);

    // write out the file
    let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
    serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
}


pub fn get_pending()->Event{

  let f_pending = File::open("./storage/pending.json").unwrap();
    let f_finish = File::open("./storage/finish.json").unwrap();
    let f_running = File::open("./storage/running.json").unwrap();

    let v_pending: serde_json::Value = serde_json::from_reader(f_pending).unwrap();
    let v_finish: serde_json::Value = serde_json::from_reader(f_finish).unwrap();
    let v_running: serde_json::Value = serde_json::from_reader(f_running).unwrap();

    let mut arrary_pending = v_pending.as_array().unwrap().clone();
    let mut arrary_finish = v_finish.as_array().unwrap().clone();
    let mut arrary_running = v_running.as_array().unwrap().clone();

  //make sure pending not in runing and finish
  for i in 0..arrary_pending.len(){
    for j in 0..arrary_finish.len(){
      if arrary_pending.get(i).unwrap()["tracnsactionHash"]
      == arrary_finish.get(j).unwrap()["tracnsactionHash"]{
         println!("{:?}",arrary_pending.get(i).unwrap());
         arrary_pending.remove(i);
         // write out the file
        let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
        serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
      }
     
    }
  }

  for i in 0..arrary_pending.len(){
    for j in 0..arrary_running.len(){
      if arrary_pending.get(i).unwrap()["tracnsactionHash"]
      == arrary_running.get(j).unwrap()["tracnsactionHash"]{
        
        println!("{:?}",arrary_pending.get(i).unwrap());
        arrary_pending.remove(i);
        let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
        serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
      }  
    }
  }

  let event = Event{
    balance: arrary_pending.get(0).unwrap()["balance"].as_str().unwrap().parse::<f32>().unwrap(),
    blocknumber: arrary_pending.get(0).unwrap()["blocknumber"].as_str().unwrap().parse::<i32>().unwrap(),
    dexaddress: arrary_pending.get(0).unwrap()["dexaddress"].as_str().unwrap().to_string(),
    model: arrary_pending.get(0).unwrap()["model"].as_str().unwrap().to_string(),
    serveraddress: arrary_pending.get(0).unwrap()["serveraddress"].as_str().unwrap().to_string(),
    tracnsactionhash: arrary_pending.get(0).unwrap()["tracnsactionhash"].as_str().unwrap().to_string(),
    useraddress: arrary_pending.get(0).unwrap()["useraddress"].as_str().unwrap().to_string()
  };

  return event;
}

pub  fn creat_server(event:Event){


  let web3_event_usdt = event.balance;
  let web3_event_model = event.model;
  let web3_event_dexaddress =event.dexaddress;
  let web3_event_useraddress =event.useraddress;
  let web3_event_serveraddress =event.serveraddress;

  //update function just need to modify here
  if web3_event_model == "AIP30"{
    let (centrial_sender, server_reciver) = channel();
    let (server_sender, centrial_reciver) = channel();
    let controler = thread::spawn(move||Server::AIP_30(server_reciver, server_sender));
    return; controler;
  }else if web3_event_model == "AIP" {
      
  }else if web3_event_model == "GRIDGLMR" {
      
  }

}

fn updat_option_by_station(){
  
}







