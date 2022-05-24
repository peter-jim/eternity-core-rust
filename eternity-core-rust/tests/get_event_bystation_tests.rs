#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::{thread::JoinHandle, sync::mpsc::Receiver,sync::mpsc::Sender};
    use eternity_core_rust::market::*;
    use eternity_core_rust::api::*;
    use eternity_core_rust::account::*;
    use eternity_core_rust::mpscanaly::*;
    use hmac::digest::generic_array::typenum::Length;
    use reqwest::StatusCode;
    use reqwest::blocking::Response;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
    use serde::Serialize;
    use std::io::BufWriter;
    use std::io::BufReader;
    
    #[test]
     fn reqwets_test(){
        
        #[derive(Serialize)]
        struct Event<'a> {
            nodeaddress: &'a str,
            body: &'a str,

        }
       
       let client = reqwest::blocking::Client::builder()
       .pool_idle_timeout(None)
       .build()
       .unwrap();

       let response = client.get("http://127.0.0.1:5000/chaindata").json(&Event{
        nodeaddress: "rust",
        body: "json",
       }).send().ok();
    //    println!("{:?}",&response.unwrap());   

    //   println!("{:?}",response.unwrap().text());
       
    //    println!("{:?}",&response.unwrap().json::<serde_json::Value>().unwrap());
    //    let mut array = response.unwrap().json::<serde_json::Value>().unwrap().as_array().unwrap().clone();
       let mut array = response.unwrap().json::<serde_json::Value>().unwrap().clone();
       
   
       println!("{:?}",array);

       for i in 0..4{
            println!("{:?}",array.get(i).unwrap()["balance"].as_f64().unwrap());
       }
       

       let f = File::open("conf.json").unwrap();
       let v: serde_json::Value = serde_json::from_reader(f).unwrap();


         // write out the file
        let writer = BufWriter::new(File::create("./storage/pending.json").unwrap());
        serde_json::to_writer_pretty(writer, &array).unwrap();


    }

}