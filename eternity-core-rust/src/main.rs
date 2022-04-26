use std::error::Error;
use eternity_core_rust::api::*;
use eternity_core_rust::market::*;
use eternity_core_rust::account::*;

fn main() -> Result<(), Box<dyn Error>> {

    let  url: String = String::from(API::Spot(Spot::Depth));
    print!("{:?}\n",url);

    let market: Market = Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));
      // Latest price for ONE symbol
      match market.get_price("BNBUSDT") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let account:Account =  Binance::new(Option::Some(String::from("y5r59DKiJ1b6MvJmxRhhDSjcAmsf5blzdqIhjGpudvrEmurVu0KJXUCdqoQpcxBx")),Option::Some(String::from("GEhNOnOBARV3NdSZRk2w6uw0qjJIWTBYSOBk7f4UzmcGPurzh6qU4YC0sbSfJgiA")));

      match account.get_account() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())



    // let client = Client::new();
    // let resp = client.get("https://api2.binance.com/api/v3/ticker/price?symbol=BNBUSDT").send()?;
    // println!("Got {:?}", resp);



}
