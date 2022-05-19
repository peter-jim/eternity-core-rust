#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::market::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
    use eternity_core_rust::mpscanaly::*;
    use reqwest::StatusCode;
    use reqwest::blocking::Response;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
    use serde::Serialize;
    use std::fs::File;
        
    #[test]
     fn event_pramster_enum_test(){
        
    
        #[derive(Debug)]
        pub enum EventCode{
        
            Withdraw{name:String ,address:String, serveraddress:String },
            Modelserver{name:String ,address:String, serveraddress:String },
            None,
        
        }
       
        let event = EventCode::Withdraw{name:"AIP".to_string(),address:"0x123".to_string(),serveraddress:"0xabc123".to_string()};


        match event {
            EventCode::Withdraw{name,address,serveraddress }  =>{
                println!("Withdraw {:?}",&name);

            }
            EventCode::Modelserver{name,address,serveraddress }  =>{
                println!("Modelserver {:?}",&name);

            }

            _ => {
                println!("none ");
            }
            
        }

    }

    #[derive(Debug)]
    struct EventInfo{
        web3_event_usdt:String,
        web3_event_model:String,
        web3_event_dexaddress:String,
        web3_event_useraddress:String,
        web3_event_serveraddress:String
    }

    // #[test]
     fn event_pramster_test() -> EventInfo{
       
        
        //获取到的链上mock 数据
        let web3_event_usdt = String::from("100");
        let web3_event_model = String::from("AIP30");
        let web3_event_dexaddress = String::from("binance");
        let web3_event_useraddress = String::from("0x123");
        let web3_event_serveraddress = String::from("0xabc123");

        //判断链上 serveraddress 与本地的服务地址是否相同，如果相同则代表是我们的服务。
        let f = File::open("./conf.json").unwrap();
        let v: serde_json::Value = serde_json::from_reader(f).unwrap();
        println!("{:?}", v["name"].as_str().unwrap());
        println!("{:?}", v["binance"]["model"]);
        
        for i in v["binance"]["model"].as_object().unwrap(){
            println!("{:?}  ",&i.0);

            if web3_event_model == i.0.clone(){
                println!("we have server");

                
                let event =  EventInfo{
                    web3_event_usdt : String::from("100"),
                     web3_event_model : String::from("AIP30"),
                     web3_event_dexaddress : String::from("binance"),
                     web3_event_useraddress : String::from("0x123"),
                     web3_event_serveraddress : String::from("0xabc123")
                };
                return event

            }
        }

       

        let event =  EventInfo{
            web3_event_usdt : String::from("100"),
             web3_event_model : String::from("AIP30"),
             web3_event_dexaddress : String::from("binance"),
             web3_event_useraddress : String::from("0x123"),
             web3_event_serveraddress : String::from("0xabc123")
        };
        return  event


     }

}