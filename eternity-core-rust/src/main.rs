use eternity_core_rust::account::*;
use eternity_core_rust::api::*;
use eternity_core_rust::event::*;
use eternity_core_rust::market::*;
use eternity_core_rust::mpscanaly::*;
use eternity_core_rust::server::*;
use eternity_core_rust::mysql::*;
use eternity_core_rust::pystation::*;
use hmac::digest::consts::False;
use hmac::digest::consts::True;
use mysql::from_row;
use mysql::params;
use mysql::prelude::*;
use mysql::Opts;
use mysql::Pool;
use mysql::PooledConn;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::f32::consts::E;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;


fn main() -> Result<(), Box<dyn Error>> {
    clean_mysql_running();

    let mut server_list = Vec::new();

    loop {
        // updata_conf();
        let f = File::open("conf.json").unwrap();
        let v: serde_json::Value = serde_json::from_reader(f).unwrap();
        let serveraddress = v["binance"]["serveraddress"].clone();

        update_event_by_station(serveraddress.clone());
        update_option_by_station(serveraddress);

       
        
        let server = process_event();
        let option = process_option();

        //处理返回server,将结果添加至list表。
        match server {
            Ok(_) => {
                server_list.push(server.unwrap());
            },
            Err(_) => {
                println!("process event 返回{:?}",server.err())
            },
        }

        match option {
            Ok(_) => {
                let e = option.unwrap().transactionhash;
                for i in 0..server_list.len(){
                    if e == server_list[i].transactionhash{
                        server_list[i].centrial_sender.clone().send(OptionCode::Withdraw);
                    }
                }         
            },
            Err(_) => {
                println!("process option 返回{:?}",option.err())
            },
        }
        
        println!("当前有{:?}个线程在运行",server_list.len());

        // let event_result = get_pending_v2();
        // let option_result = get_option_code_v2();

        // let num = server_list.len() as i32;
        // let c= max_server_check_v2(num);

        // //
        // if max_server_check_v2(num) == true {
        //     if event_result.is_ok() {
        //         let event = event_result.ok().unwrap();
        //         let id = creat_server(event);
        //         server_list.push(id);
        //     } else {
        //         println!("没有新业务，目前有{:?}个线程在运行", server_list.len());
        //     }
        // } else {
        //     println!("目前的最大负载数已达上限请更新服务器配置，并在conf.json中扩容");
        // }

        // match c {
        //     true => {
                
        //         if &event_result.clone().is_ok() {
        //             let event = event_result.ok().unwrap();
        //             let id = creat_server(event);
        //             server_list.push(id);
        //         } else {
        //             println!("没有新业务，目前有{:?}个线程在运行", server_list.len());
        //         }
        //     },

        //     false => {
        //         println!("目前的最大负载数已达上限请更新服务器配置，并在conf.json中扩容");
        //     }  
        // }


        // if option_result.is_ok() {
        //     let option = option_result.ok().unwrap();

        //     println!(" opiton is {:?}", option.transactionhash);
        //     println!(" server_list is {:?}", server_list);

        //     let mut flage = false;
        //     // fina a bug ,if op pending not run in pending threding ,it not wrok
        //     for i in &server_list {
        //         println!(" server option is {:?}", i.transactionhash);
        //         if i.transactionhash == option.transactionhash {
        //             println!("  找到一个可执行 Option   ");
        //             creat_option(option.clone());
        //             let op_sender = i.centrial_sender.clone();
        //             op_sender.send(OptionCode::Withdraw);

        //             // std::thread::sleep(std::time::Duration::from_secs(3));

        //             flage = true;
        //         }
        //     }

        //     if flage == false {
        //         //如果不在runin，代表该服务还未启动。1.我们需要启动该服务 2.移除该服务，
        //         println!(
        //             " 该option {:?} 未在running中找到程序",
        //             option.transactionhash
        //         );
        //     }
        // } else {
        //     println!("没有 需要操作 的 ChainOption");
        // }

        // 获取操作码，1.提现
        // let op_code =  get_option_code();

        std::thread::sleep(std::time::Duration::from_secs(10));

        //返回函数全局状态到中性化节点
        
    }

    Ok(())
}

fn clean_running() {
    //启动或重启，先清空op_running，和running。因为目前为止没有对应的线程。
    let mut empty_array = serde_json::Value::Array(Vec::new())
        .as_array()
        .unwrap()
        .clone();

    let writer = BufWriter::new(File::create("./storage/op_running.json").unwrap());
    serde_json::to_writer_pretty(writer, &empty_array).unwrap();

    let mut empty_array = serde_json::Value::Array(Vec::new())
        .as_array()
        .unwrap()
        .clone();

    let writer = BufWriter::new(File::create("./storage/running.json").unwrap());
    serde_json::to_writer_pretty(writer, &empty_array).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn max_server_check(num: i32) -> bool {
    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();

    let maxaccount = v["binance"]["maxaccount"].as_i64().unwrap() as i32;

    let f_running = File::open("./storage/running.json").unwrap();
    let v_running: serde_json::Value = serde_json::from_reader(f_running).unwrap();
    let mut arrary_running = v_running.as_array().unwrap().clone();
    let num_run = arrary_running.len() as i32;

    if num_run < maxaccount {
        return true;
    } else {
        return false;
    }
}

fn max_server_check_v2(num: i32)  -> bool{

    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let maxaccount = v["binance"]["maxaccount"].as_i64().unwrap() ;

    let mut conn = init_mysql();
    let mut res:Vec<(String,String,String,f32,String,String,String,String)> = conn
    .query(
        "select * from NodeAccountStatus where eventstatus = 'running' "
    ).unwrap();
    println!("存在数据  {:?}",res); 
    
    let num_run = res.len() as i64;
  


    if num_run < maxaccount {
        return true;
    } else {
        return false;
    }
}


fn updata_conf() {
    println!(" update config ");
    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let serveraddress = v["binance"]["serveraddress"].clone();
    
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

        let mut array_result = response.unwrap().json::<serde_json::Value>();

        if array_result.is_ok() {
            let mut array = array_result.unwrap().as_array().unwrap().clone();

            let mut array_event = Vec::new();

            for i in 0..array.len() {
                let e = eternity_core_rust::event::Event {
                    balance: array[i]["balance"].as_f64().unwrap() as f32,
                    
                    dexaddress: array[i]["dexaddress"].as_str().unwrap().to_string(),
                    model: array[i]["model"].as_str().unwrap().to_string(),
                    serveraddress: array[i]["serveraddress"].as_str().unwrap().to_string(),
                    transactionhash: array[i]["transactionHash"].as_str().unwrap().to_string(),
                    useraddress: array[i]["useraddress"].as_str().unwrap().to_string(),
                    
                    optionstatus: todo!(),
                    eventstatus: todo!(),
                };

                array_event.push(e);
            }

            // println!("array is   {:?}", array.len());
            // println!(
            //     "array transactionHash is   {:?}",
            //     array[0]["transactionHash"]
            // );

            //array must not in finish , pending anding running. If
            for i in 0..array.clone().len() {
                for j in 0..arrary_finish.clone().len() {
                    println!("arrary finish is {:?}", arrary_finish[j]["transactionHash"]);
                    if array[i]["transactionHash"] == arrary_finish[j]["transactionHash"] {
                        // array.remove(i);
                        println!("在finish 中发现1个重复");
                        repeate_array.push(array[i].clone());
                    }
                }
            }

            // not in runing
            for i in 0..array.clone().len() {
                for j in 0..arrary_running.len() {
                    if array[i]["transactionHash"] == arrary_running[j]["transactionHash"] {
                        // println!("在runing 中发现1个重复");
                        repeate_array.push(array[i].clone());
                    }
                }
            }

            // println!("array out for is {:?}", &array.len());
            //not in pending

            //find a bug if
            for i in 0..array.clone().len() {
                //  println!(" array in for is {:?}", array[i]["transactionHash"]);
                for j in 0..arrary_pending.clone().len() {
                    // println!(" repeate in for is {:?}", arrary_pending[j]["transactionHash"]);
                    if array[i]["transactionHash"] == arrary_pending[j]["transactionHash"] {
                        // println!("在 pending中发现1个重复 {:?}" , array[i]["transactionHash"] );
                        repeate_array.push(array[i].clone());
                    }
                }
            }

            //
            for i in 0..repeate_array.clone().len() {
                for j in 0..array.len() {
                    // println!("xx repeate_array 是{:?}",repeate_array[i]["transactionHash"]);
                    //     println!("xx array 是{:?}",array[j]["transactionHash"]);
                    if repeate_array[i]["transactionHash"] == array[j]["transactionHash"] {
                        // println!("在 repeated 和 array 中发现一个重复");
                        // println!("repeate_array 是{:?}",repeate_array[i]["transactionHash"]);
                        // println!("array 剩 {:?}", array);
                        // array.remove(j);
                        // array_event[j].cheakcode = false;
                    }
                }
            }

            // println!("重复的数据有{:?} 个", repeate_array.len());

            if array.len() != 0 {
                for i in 0..array_event.len() {
                    // if array_event[i].cheakcode == true {
                    //     arrary_pending.push(array[i].clone());
                    // }
                }

                // write out the file
                let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
                serde_json::to_writer_pretty(writer, &arrary_pending).unwrap();
            } else {
                println!("不需要更新")
            }
        } else {
            println!(" 网络错误,无法连接到监听节点 ");
        }
    } else {
        println!("network error");
    }
}

pub fn get_pending() -> Result<Event, String> {
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

    if arrary_pending.len() != 0 {
        // println!("array pending balance {:?} ", arrary_pending[0]["balance"]);
        let event = Event {
            balance: arrary_pending[0]["balance"].as_f64().unwrap() as f32,
           
            dexaddress: arrary_pending[0]["dexaddress"]
                .as_str()
                .unwrap()
                .to_string(),
            model: arrary_pending[0]["model"].as_str().unwrap().to_string(),
            serveraddress: arrary_pending[0]["serveraddress"]
                .as_str()
                .unwrap()
                .to_string(),
            transactionhash: arrary_pending[0]["transactionHash"]
                .as_str()
                .unwrap()
                .to_string(),
            useraddress: arrary_pending[0]["useraddress"]
                .as_str()
                .unwrap()
                .to_string(),
            optionstatus: todo!(),
            eventstatus: todo!(),

        };
        return Result::Ok(event);
    }

    return Result::Err("no pending".to_string());
}

pub fn get_pending_v2() ->Result<Event, String>{
    let mut conn = init_mysql();
  
    let mut res:Vec<(String,String,String,f32,String,String,String,String)> = conn
    .query(
        "select * from NodeAccountStatus where optionstatus = 'null' "
    ).unwrap();
    println!("存在数据  {:?}",res); 

    
    if res.len() > 0 {
        println!("查询为空");
        return Result::Err("no pending".to_string());
    } else {
        println!("已经存在数据");

        let event = Event{
            balance:res[0].3 ,
            
            dexaddress:res[0].1.clone(),
            model:res[0].6.clone(),
            serveraddress:res[0].2.clone(),
            transactionhash:res[0].0.clone(),
            useraddress:res[0].7.clone(),
            optionstatus: todo!(),
            eventstatus: todo!(),
            
        };
        return Result::Ok(event);
    }

    println!(" pending is {:?}" , res);
    
    
}

