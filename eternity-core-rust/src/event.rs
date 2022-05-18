
#[derive(Debug)]
pub enum EventCode{

    Withdraw{name:String ,address:String, serveraddress:String },
    Modelserver{name:String ,address:String, serveraddress:String },

}


fn a(){
    let b = EventCode::Withdraw{name:"AIP",address:"0x123",serveraddress:"0xabc123"};
    
    
}



