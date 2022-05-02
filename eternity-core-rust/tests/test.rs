



#[cfg(test)]
mod tests {
    use std::{thread::JoinHandle, sync::mpsc::Receiver};

    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Sender ;
    use std::fs::File;
    extern crate serde;
extern crate serde_json;

    struct Server{
        quant_id:String,
        thrding: JoinHandle<()>,
        start_time: String,
        account:String,
        reciver:Receiver<String>,
        sender:Sender<String>,
        
    }

    impl Server {
        fn data(num:i32){
            let a = num;
            println!(" a is {}",&a);
        }
    }

    fn data(num:i32){
        let a = num;
        println!(" a is {}",&a);
    }

    fn quant(num:String) {
        let a = std::thread::spawn(move|| {
            for i in 1..10 {
               println!("hi number {} from the spawned thread!  num is {}", i,num);
               std::thread::sleep(std::time::Duration::from_millis(1));
            }
         });
    }




    #[test]
    fn thriding() {
        
      let mut stack = Vec::new();
      stack.push(std::thread::spawn(move||quant("1".to_string())));
      stack.push(std::thread::spawn(move||quant('2'.to_string())));
      stack.push(std::thread::spawn(move||quant('3'.to_string())));
      std::thread::sleep(std::time::Duration::from_millis(10));

      let (tx, rx) = channel();
      tx.send("t".to_string());

      println!("rx is {:?}",rx.recv());
      tx.send("t".to_string());

      println!("rx is {:?}",rx.recv());tx.send("t".to_string());

      println!("rx is {:?}",rx.recv());
      


      let server1 =  Server{
        quant_id:"s".to_string(),
        thrding: std::thread::spawn(move|| Server::data(1)),
        start_time: "2021".to_string(),
        account:"账户1".to_string(),
        reciver:rx,
        sender:tx,
      };

    //   let server2 =  Server{
    //     quant_id:"s".to_string(),
    //     thrding: std::thread::spawn(move||data(12)),
    //     start_time: "2021".to_string(),
    //     account:"账户1".to_string()
    //     reciver:rx,
    //     sender:tx,
    //   };

    //   server1.thrding;
    //   server2.thrding;

    //   we cant run agint because of server thrding is move.
    //   server1.thrding;
    //   server2.thrding;
      


    }

    #[test]
    fn json(){
        let f = File::open("conf.json").unwrap();
        let v: serde_json::Value = serde_json::from_reader(f).unwrap();
        println!("{:?}", v["name"].as_str().unwrap());


        let web3_event_info = String::from("AIP");
        for i in v["binance"]["model"].as_array().unwrap(){
            
            if &i.as_str().unwrap() == &web3_event_info{
                println!("{:?}  --  {:?}",&i.as_str().unwrap(),&web3_event_info);
            }
         
        }
    }


}