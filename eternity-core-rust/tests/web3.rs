#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use secp256k1::SecretKey;
    use web3::{
        contract::{Contract, Options},
        types::H160, api::Accounts
    };
    use std::{time, str::FromStr};
        
   
   
     fn event_pramster_enum_test()->web3::contract::Result<()> {
        println!("Accounts: ");
        let transport = web3::transports::Http::new("https://rpc.testnet.moonbeam.network");
        let web3 = web3::Web3::new(transport.unwrap());
        

        let private_key = SecretKey::from_str("e7688610e0ebfccbac5c9c5d637db2910d4b64f6f36460de6b964f4c725c9f95").unwrap();
  

        let contract = Contract::from_json(web3.eth(), hex!("0247AaB1D43c4A40119ea057f2ab5c631ead8D2f").into(),include_bytes!("../res/demo.abi") ).unwrap();
    
    
        let mut accounts = web3.eth().accounts();
        // accounts.push(hex!("ba7fb8146b2ae3E335332712CDE2C044E57f80A9").into());

        
        
        Ok(())
        
        
    
     }

}