
#[derive(Debug)]
pub enum EventCode{

    Withdraw{name:String ,address:String, serveraddress:String },
    Modelserver{name:String ,address:String, serveraddress:String },

}

#[derive(Debug,Clone)]
pub struct  Event{
    pub balance: f32,
    pub blocknumber: i32,
    pub dexaddress: String,
    pub model: String,
    pub serveraddress: String,
    pub transactionhash: String,
    pub useraddress: String,
    pub cheakcode: bool,

}





