use web3::contract::{Contract,Options};
use web3::transports::Http;
use web3::types::{TransactionParameters,Address,};
use web3::{Web3};
use dotenv::dotenv;
use std::env;
use std::fs;
use serde_json::{Result};
use web3::signing::SecretKey;

pub async fn save_certificate(sig_hash: &str) -> Result<String> {
    dotenv().ok();
    let node_url = env::var("NODE_URL").unwrap();
    let contract_address:Address = env::var("CONTRACT_ADDRESS").unwrap().parse().unwrap();
    let private_key: SecretKey = env::var("PRIVATE_KEY").unwrap().parse().unwrap();
    let wallet_address:Address=env::var("WALLET_ADDRESS").unwrap().parse().unwrap();

    let abi_json=fs::read_to_string("./abi.json").expect("Unable to read abi file");
    let contract_abi = serde_json::from_str(&abi_json)?;

    let transport = Http::new(&node_url).unwrap();
    let web3 = Web3::new(transport);

    

    let contract = Contract::new(web3.eth(), contract_address,contract_abi);
    let call=contract.call("storeHash", vec![sig_hash.as_bytes().to_vec()],wallet_address,Options::default()).await;
    let tx=TransactionParameters{
        to: Some(contract_address),
        gas: 100000.into(),
        data: call.unwrap().as_bytes().into(),
        chain_id: Some(80002),
        ..Default::default()
    };


    let signed_tx = futures::executor::block_on(web3.accounts().sign_transaction(tx, &private_key));
    let tx_hash =web3.eth().send_raw_transaction(signed_tx.unwrap().raw_transaction).await;

    return Ok(tx_hash.unwrap().to_string());
}

