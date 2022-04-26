use std::thread as thread;
use std::time::Duration;
use std::sync::mpsc as mpsc;

#[cfg(test)]
mod tests {
 
    #[test]
    fn thriding() {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

}