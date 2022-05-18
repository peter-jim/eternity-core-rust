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
        
    #[test]
     fn reqwets_test(){
        
        #[derive(Serialize)]
        struct Body<'a> {
            lang: &'a str,
            body: &'a str,
        }
       
       let client = reqwest::blocking::Client::builder()
       .pool_idle_timeout(None)
       .build()
       .unwrap();

       let response = client.get("http://127.0.0.1:5000/").json(&Body{
        lang: "rust",
        body: "json",
       }).send().ok();
       println!("{:?}",response.unwrap());   

       


    }

}