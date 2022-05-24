use eternity_core_rust::account::*;
use eternity_core_rust::api::*;
use eternity_core_rust::event::*;
use eternity_core_rust::market::*;
use eternity_core_rust::mpscanaly::*;
use eternity_core_rust::server::*;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        updata_conf();
        let event = get_pending();
        // let id = creat_server(event);
        println!(" loop ");

        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    Ok(())
}

fn updata_conf() {
    println!(" update config ");
    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let serveraddress = v["binance"]["serveraddress"].clone();
    updata_event_by_station(serveraddress);
}

fn updata_event_by_station(serveraddress: Value) {
    println!(" get event by station");
    let f_pending = File::open("./storage/pending.json").unwrap();
    let f_finish = File::open("./storage/finish.json").unwrap();
    let f_running = File::open("./storage/running.json").unwrap();

    let v_pending: serde_json::Value = serde_json::from_reader(f_pending).unwrap();
    let v_finish: serde_json::Value = serde_json::from_reader(f_finish).unwrap();
    let v_running: serde_json::Value = serde_json::from_reader(f_running).unwrap();

    let mut arrary_pending = v_pending.as_array().unwrap().clone();
    let mut arrary_finish = v_finish.as_array().unwrap().clone();
    let mut arrary_running = v_running.as_array().unwrap().clone();

    let mut repeate_array = serde_json::Value::Array(Vec::new())
        .as_array()
        .unwrap()
        .clone();
    println!("pending is {:?}", arrary_pending);

    #[derive(Serialize)]
    struct Event<'a> {
        nodeaddress: &'a str,
        body: &'a str,
    }

    let client = reqwest::blocking::Client::builder()
        .pool_idle_timeout(None)
        .build()
        .unwrap();

    println!(" ready to get chain event data  ");
    let response = client
        .get("http://127.0.0.1:5000/chaindata")
        .json(&Event {
            nodeaddress: serveraddress.as_str().unwrap(),
            body: "json",
        })
        .send();
    println!(" get response status is  {:?}", response.is_ok());

    if response.is_ok() == true {
        let response = response.ok();

        let mut array = response
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap()
            .as_array()
            .unwrap()
            .clone();

        println!("array is   {:?}", array.len());
        println!(
            "array transactionHash is   {:?}",
            array[0]["transactionHash"]
        );

        //array must not in finish , pending anding running. If
        for i in 0..array.clone().len() {
            for j in 0..arrary_finish.len() {
                if array[i]["transactionHash"] == arrary_finish[j]["transactionHash"] {
                    // array.remove(i);
                    repeate_array.push(array[i]["transactionHash"].clone());
                }
            }
        }

        // not in runing
        for i in 0..array.clone().len() {
            for j in 0..arrary_running.len() {
                if array[i]["transactionHash"] == arrary_running[j]["transactionHash"] {
                    repeate_array.push(array[i]["transactionHash"].clone());
                }
            }
        }

        // println!("array out for is {:?}", &array.len());
        //not in pending

        //find a bug if
        for i in 0..array.clone().len() {
            // println!(" array in for is {:?}", &array.len());
            for j in 0..arrary_pending.clone().len() {
            

                if array[i]["transactionHash"] == arrary_pending[j]["transactionHash"] {

                    repeate_array.push(array[i]["transactionHash"].clone());
                }
            }
        }

        //
        for i in 0..repeate_array.clone().len(){
            for j in 0..array.len(){
                if repeate_array[i]["transactionHash"] == array[j]["transactionHash"]{
                    array.remove(j);
                }
            }
        }

        println!(" 没有重复的数据有 {:?} ",array.len() );

        if array.len() !=0 {
            arrary_pending.push(array[0].clone());

            // write out the file
            let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
            serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
        }else {
            println!("不需要更新")
        }
        
    } else {
        println!("network error");
    }
}

pub fn get_pending() -> Event {
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
    for i in 0..arrary_pending.len() {
        for j in 0..arrary_finish.len() {
            if arrary_pending.get(i).unwrap()["transactionHash"]
                == arrary_finish.get(j).unwrap()["transactionHash"]
            {
                println!("{:?}", arrary_pending.get(i).unwrap());
                arrary_pending.remove(i);
                // write out the file
                let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
                serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
            }
        }
    }

    for i in 0..arrary_pending.len() {
        for j in 0..arrary_running.len() {
            if arrary_pending.get(i).unwrap()["transactionHash"]
                == arrary_running.get(j).unwrap()["transactionHash"]
            {
                println!("{:?}", arrary_pending.get(i).unwrap());
                arrary_pending.remove(i);
                let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
                serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
            }
        }
    }

    println!("array pending balance {:?} ", arrary_pending[0]["balance"]);

    let event = Event {
        balance: arrary_pending[0]["balance"].as_f64().unwrap() as f32,
        blocknumber: arrary_pending[0]["blocknumber"].as_f64().unwrap() as i32,
        dexaddress: arrary_pending[0]["dexaddress"]
            .as_str()
            .unwrap()
            .to_string(),
        model: arrary_pending[0]["model"].as_str().unwrap().to_string(),
        serveraddress: arrary_pending[0]["serveraddress"]
            .as_str()
            .unwrap()
            .to_string(),
        tracnsactionhash: arrary_pending[0]["transactionHash"]
            .as_str()
            .unwrap()
            .to_string(),
        useraddress: arrary_pending[0]["useraddress"]
            .as_str()
            .unwrap()
            .to_string(),
    };

    return event;
}

pub fn creat_server(event: Event) -> Server {
    let web3_event_usdt = event.balance;
    let web3_event_model = event.model;
    let web3_event_dexaddress = event.dexaddress;
    let web3_event_useraddress = event.useraddress;
    let web3_event_serveraddress = event.serveraddress;

    //update function just need to modify here
    if web3_event_model == "AIP30" {
        let (centrial_sender, server_reciver) = channel();
        let (server_sender, centrial_reciver) = channel();
        let controler = thread::spawn(move || Server::AIP_30(server_reciver, server_sender));
        let server = Server {
            quant_id: 1,
            threading: controler,
            start_time: "2021".to_string(),
            account: "web3_event_useraddress".to_string(),

            server_reciver: centrial_reciver,
            centrial_sender: centrial_sender,
        };
        return server;
    } else {
        let (centrial_sender, server_reciver) = channel();
        let (server_sender, centrial_reciver) = channel();
        let controler = thread::spawn(move || Server::AIP_30(server_reciver, server_sender));
        let server = Server {
            quant_id: 1,
            threading: controler,
            start_time: "2021".to_string(),
            account: "web3_event_useraddress".to_string(),

            server_reciver: centrial_reciver,
            centrial_sender: centrial_sender,
        };
        return server;
    }
}

fn updat_option_by_station() {

    //   let client = reqwest::blocking::Client::builder()
    //   .pool_idle_timeout(None)
    //   .build()
    //   .unwrap();

    //   let response = client
    //         .get("http://127.0.0.1:5000/chaindata")
    //         .json(&EventCode {
    //             nodeaddress: serveraddress.as_str().unwrap(),
    //             body: "json",
    //         })
    //         .send()
    //         .ok();

    //发送消息
    //  stack[0].centrial_sender.send(OptionCode::Shoutdown);

    //  loop {
    //      //发送消息
    //      stack[0].centrial_sender.send(OptionCode::ErrorStatus);
    //      std::thread::sleep(std::time::Duration::from_secs(3));

    //      println!("{:?} 主程序消息   ", stack[0].server_reciver.recv())}
}

fn get_option_code() {}

fn send_option_code_to_server() {}
