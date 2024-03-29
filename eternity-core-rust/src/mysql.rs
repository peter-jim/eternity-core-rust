use std::fs::File;

use mysql::params;
use mysql::prelude::*;
use mysql::Opts;
use mysql::Pool;
use mysql::PooledConn;
use serde_json::Value;

use crate::event::Event;
use crate::server::Server;
use crate::server::create_server;

pub fn init_mysql() -> PooledConn {
    let f = File::open("conf.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let url = v["mysql"].as_str().unwrap();
    // println!("初始化muysql");
    //设置连接字符串
    // let url = "mysql://root:1416615127dj@localhost:3306/event";
    let opts = Opts::from_url(url).unwrap(); // 类型转换将 url 转为opts
                                             //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}

pub fn clean_mysql_running() {
    let mut conn = init_mysql();

    //当机器重新启动时候，更新mysql的状态,running 状态的程序需要复原，到再次重启。
    let res:Vec<(String,String,String,f32,String,String,String,String)> =
        conn.query(
            "update NodeAccountStatus SET eventstatus = 'pending' where eventstatus = 'running'  "
        ).unwrap();
    // println!("更新Option 数据  {:?}", res);

    let res:Vec<(String,String,String,f32,String,String,String,String)> =conn.query(
        "update NodeAccountStatus SET optionstatus = 'pending' where optionstatus = 'running'  "
    ).unwrap();

}

//根据transaction hash 判断
fn is_exist_in_mysql(event: Value) -> bool {
    let mut conn = init_mysql();
    let e = event;
    //数据库操作
    //1.查询user表
    //方式1：流式查询  数据逐行读取，数据不会存储在内存中

    // println!("检查是否存在transaction = {:?}", e["transactionhash"].as_str().unwrap());

    let  res: Result<Option<(String, String, String, f32, String, String, String, String)>, _> =
        conn.exec_first(
            r"select * from NodeAccountStatus where transactionhash = :transactionhash  ",
            params! {
                "transactionhash" => e["transactionhash"].as_str().unwrap()
            },
        );
    // println!("查询结果为  {:?}", res);
    if res.unwrap() == None {
        // println!("查询为空");
        return true;
    } else {
        // println!("已经存在数据");
        return false;
    }
}

//处理 链下安全节点获取到的event事件，并判断其是否需要更新到本地状态。
pub fn process_station_transaction(event: Value) {
    //true 代表为空， false 代表为存在数据。
    let is_exist = is_exist_in_mysql(event.clone());
    println!("{:?} is exist is {:?}  ", event["transactionhash"].as_str().unwrap().to_string()   ,is_exist);
    match is_exist {
        false => {
            println!("不需要更新")
        }

        true => {
            println!("插入");

            insert_event_mysql(event.clone())
        }
    }
}


pub fn insert_event_mysql(event: Value) {
    let e = event.clone();

    println!(
        "transactionhash is {:?}",
        &e["transactionhash"].as_str().unwrap().to_string()
    );

    println!(
        "e is {:?}",
        &e
    );

    let event = vec![Event {
        transactionhash: e["transactionhash"].as_str().unwrap().to_string(),
        dexaddress: e["dexaddress"].as_str().unwrap().to_string(),
        serveraddress: e["serveraddress"].as_str().unwrap().to_string(),
        balance: e["balance"].as_f64().unwrap() as f32,
        optionstatus: "null".to_string(),
        eventstatus: "pending".to_string(),
        model: e["model"].as_str().unwrap().to_string(),
        useraddress: e["useraddress"].as_str().unwrap().to_string(),
    }];

    let mut conn = init_mysql();

    println!("插入数据{:?}", event[0].transactionhash);
    conn.exec_batch(
        "INSERT INTO NodeAccountStatus (transactionhash, dexaddress, serveraddress,balance,optionstatus,eventstatus,model,useraddress)
          VALUES (:transactionhash, :dexaddress, :serveraddress,:balance,:optionstatus,:eventstatus,:model,:useraddress)",
        event.iter(). map(|p| params! {
            "transactionhash" => &p.transactionhash,
            "dexaddress" => &p.dexaddress,
            "serveraddress" => &p.serveraddress,
            "balance"=>p.balance,
            "optionstatus"=>&p.optionstatus,
            "eventstatus"=>&p.eventstatus,
            "model"=>&p.model,
            "useraddress"=>&p.useraddress,
        })
    ).unwrap();
    println!("数据更新完成");
    
}

pub fn update_option_null(event: Value) {
    let mut conn = init_mysql();

    let event = event;

    println!(
        "Option transactionhash is {:?} ",
        event["transactionhash"].as_str().unwrap().to_string()
    );

    let  res: Result<Option<(String, String, String, f32, String, String, String, String)>, _> =
        conn.exec_first(
            r"update NodeAccountStatus SET optionstatus = 'pending' where optionstatus = 'null' and transactionhash= :transactionhash  ",
            params! {
                "transactionhash" => event["transactionhash"].as_str().unwrap().to_string()
            },
        );
    println!("更新Option 数据  {:?}", res);
}

pub fn update_option_pending(event: String) {
    //更新pending -> running
    //must be mut,otherwise it show error
    let mut conn = init_mysql();


    let  res: Result<Option<(String, String, String, f32, String, String, String, String)>, _> =
        conn.exec_first(
            r"update NodeAccountStatus SET optionstatus = 'running' where optionstatus = 'pending' and transactionhash= :transactionhash  ",
            params! {
                "transactionhash" => event.to_string()
            },
        );
    println!("更新Option 数据  {:?}", res);
}

pub fn update_option_running(event: String) {
    //must be mut,otherwise it show error
    let mut conn = init_mysql();

    let  res: Result<Option<(String, String, String, f32, String, String, String, String)>, _> =
        conn.exec_first(
            r"update NodeAccountStatus SET optionstatus = 'finish' where optionstatus = 'running' and transactionhash= :transactionhash  ",
            params! {
                "transactionhash" =>  event.to_string()
            },
        );
    
}

pub fn update_event_pending(event: String) {
    let mut conn = init_mysql();

    let  res: Result<Option<(String, String, String, f32, String, String, String, String)>, _> =
        conn.exec_first(
            r"update NodeAccountStatus SET eventstatus = 'running' where eventstatus = 'pending' and transactionhash= :transactionhash  ",
            params! {
                "transactionhash" => event.to_string()
            },
        );
    
}

pub fn update_event_runing(event: String) {
    let mut conn = init_mysql();

    let  res: Result<Option<(String, String, String, f32, String, String, String, String)>, _> =
        conn.exec_first(
            r"update NodeAccountStatus SET eventstatus = 'finish' where eventstatus = 'running' and transactionhash= :transactionhash  ",
            params! {
                "transactionhash" => event.to_string()
            },
        );
}

pub fn get_event_pending()->Result<Event, String>{
    let mut conn = init_mysql();
  
    let  res:Vec<(String,String,String,f32,String,String,String,String)> = conn
    .query(
        "select * from NodeAccountStatus where eventstatus = 'pending' "
    ).unwrap();
    println!("存在数据  {:?}",res); 

    match res.len() {
        
        0 =>{
            return Result::Err("no pending".to_string())
        }

        _ =>{
            let event = Event{
                balance:res[0].3 ,
                dexaddress:res[0].1.clone(),
                model:res[0].6.clone(),
                serveraddress:res[0].2.clone(),
                transactionhash:res[0].0.clone(),
                useraddress:res[0].7.clone(),
                optionstatus: "pending".to_string(),
                eventstatus: "pending".to_string(),
                
            };
            return Result::Ok(event)
        }
    }
}

 fn get_option_pending()->Result<Event, String>{
    let mut conn = init_mysql();
  
    let  res:Vec<(String,String,String,f32,String,String,String,String)> = conn
    .query(
        "select * from NodeAccountStatus where optionstatus = 'pending' and eventstatus = 'running' "
    ).unwrap();
    println!("存在数据  {:?}",res); 

    match res.len() {
        
        0 =>{
            return Result::Err("no pending".to_string());
        }

        _ =>{
            let event = Event{
                balance:res[0].3 ,
                dexaddress:res[0].1.clone(),
                model:res[0].6.clone(),
                serveraddress:res[0].2.clone(),
                transactionhash:res[0].0.clone(),
                useraddress:res[0].7.clone(),
                optionstatus: "pending".to_string(),
                eventstatus: "running".to_string(),
                
            };

            update_option_pending(event.transactionhash.clone());
            return Result::Ok(event);
        }
    }
}

//获取一条 待处理的event 
pub fn process_event()-> Result<Server,String>{
    let re = get_event_pending();
    match re.is_ok() {
        true => {
            let event = re.ok().unwrap();
            let id = create_server(event);
            
            return Result::Ok(id.unwrap())
        }
        false => {
            return Result::Err("procession error,no pending".to_string());
        },
    }
}


//获取一条待处理的option
pub fn process_option() -> Result<Event,String>{
    let re = get_option_pending();
    match re.is_ok() {
        true => {
            let event = re.ok().unwrap();  //already update option pending
            
            
            return Result::Ok(event)
            
        }
        false => {
            return Result::Err("procession no pending".to_string());
        },
    }
}


