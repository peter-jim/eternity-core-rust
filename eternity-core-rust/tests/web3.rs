
use hex_literal::*;
use secp256k1::SecretKey;
use std::{str::FromStr, time};
use web3::{
    api::Accounts,
    contract::{Contract, Options},
    futures::FutureExt,
    types::H160,
};

#[test]
fn b() {
    println!("Accounts: ");
    let result = a();
    std::thread::sleep(std::time::Duration::from_secs(10));
    
    println!("result is: ");
}

#[tokio::test]
async fn a() -> web3::contract::Result<()> {
    println!("start run web3: ");
    let transport = web3::transports::Http::new("https://rpc.testnet.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push(hex!("ba7fb8146b2ae3E335332712CDE2C044E57f80A9").into());

    let private_key =
        SecretKey::from_str("e7688610e0ebfccbac5c9c5d637db2910d4b64f6f36460de6b964f4c725c9f95")
            .unwrap();

    let contract = Contract::from_json(
        web3.eth(),
        hex!("0247AaB1D43c4A40119ea057f2ab5c631ead8D2f").into(),
        include_bytes!("../res/demo.abi"),
    )
    .unwrap();

    // let tx =  contract.signed_call("putOrder", (accounts[0],1_000_000_u64), Options::default(), & private_key).await?;

    // println!("tx is {:?}",&tx);
    // let tx = contract.call("putOrder", (accounts[0],1_000_000_u64), accounts[0], Options::default()).await?;
    // println!("got tx: {:?}", tx);

    let tx_re = contract
        .signed_call_with_confirmations(
            "putOrder",
            (accounts[0], 1_000_000_u64),
            Options::default(),
            1,
            &private_key,
        )
        .await?;

    println!("确认后的交易是 {:?}", &tx_re);

    Ok(())
}
