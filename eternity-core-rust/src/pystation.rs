use crate::mysql::*;
use reqwest::{
    blocking::{Client, Response},
    Error,
};
use serde::Serialize;
use serde_json::Value;

pub fn update_option_by_station(serveraddress: Value) {
    
    get_option_by_station(serveraddress);
}

pub fn update_event_by_station(serveraddress: Value) {
    get_event_by_station(serveraddress);
}

//从链下可信节点获取 event 数组，返回response
pub fn get_event_by_station(serveraddress: Value) {
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
}

//从链下可信节点获取 option 数组，返回response
pub fn get_option_by_station(serveraddress: Value) {
    println!(" get option by station");
    let client = Client::builder().pool_idle_timeout(None).build().unwrap();

    #[derive(Serialize)]
    struct Node<'a> {
        nodeaddress: &'a str,
        body: &'a str,
    }

    println!(" ready to get chain event data  ");
    let response = client
        .get("http://127.0.0.1:5000/option")
        .json(&Node {
            nodeaddress: serveraddress.as_str().unwrap(),
            body: "json",
        })
        .send();

    println!(" get response status is  {:?}", response.is_ok());
}

fn process_event_response(response: Result<Response, Error>) {
    match response.is_ok() {
        true => {
            let response = response.ok();
            let array_result = response.unwrap().json::<serde_json::Value>();
            if array_result.is_ok() {
                let array = array_result.unwrap().as_array().unwrap().clone();

                for i in 0..array.len() {
                    process_event_transaction(array[i].clone())
                }
            } else {
                println!(" 网络错误,无法处理 ");
            }
        }
        false => {
            println!(" 网络错误,无法连接到链下可信节点。 ");
        }
    }
}

fn process_option_response(response: Result<Response, Error>) {
    match response.is_ok() {
        true => {
            let response = response.ok();
            let array_result = response.unwrap().json::<serde_json::Value>();
            if array_result.is_ok() {
                let array = array_result.unwrap().as_array().unwrap().clone();

                for i in 0..array.len() {
                    //更新本地的option null -> pending
                    update_option_null(array[i].clone())
                }
            } else {
                println!(" 网络错误,无法处理 ");
            }
        }
        false => {
            println!(" 网络错误,无法连接到链下可信节点。 ");
        }
    }
}
