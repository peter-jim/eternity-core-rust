use std::error::Error;
use eternity_core_rust::api::*;

fn main() -> Result<(), Box<dyn Error>> {

    let  url: String = String::from(API::Spot(Spot::Depth));
    print!("{:?}",url);
    Ok(())
}