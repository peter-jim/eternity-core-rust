#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::market::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
    use eternity_core_rust::mpscanaly::*;
    use serde::Serialize;
    use reqwest::StatusCode;
    use reqwest::blocking::Response;
    #[test]
    fn option_test(){
       
        #[derive(Serialize)]
        struct Node<'a> {
            nodeaddress: &'a str,
            body: &'a str,
        }
        
        println!(" ready to get chain event data  ");
        let client = reqwest::blocking::Client::builder()
        .pool_idle_timeout(None)
        .build()
        .unwrap();
 
        let response = client.get("http://127.0.0.1:5000/chaindata").json(&Node{
         nodeaddress: "rust",
         body: "json",
        }).send().ok();


        println!(" get response status is  {:?}", response);

    }

}