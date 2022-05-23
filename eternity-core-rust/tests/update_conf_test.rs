#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::server::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
        
    #[test]
    fn update_conf_test(){
        let client = reqwest::blocking::Client::builder()
        .pool_idle_timeout(None)
        .build()
        .unwrap();
 
        let response = client.get("http://127.0.0.1:5000/update").send().ok();
        // println!("{:#?}",&response.unwrap().json::<serde_json::Value>().ok());  

        let model_json = response.unwrap().json::<serde_json::Value>().unwrap();
        println!("{:?}",model_json["binance"])

    }

}
