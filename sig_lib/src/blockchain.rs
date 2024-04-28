use web3::contract::{Contract, Options};
use web3::types::{Address, Transaction, U256};
use web3::transports::{Http};
use std::env;

let node_url = "https://rpc-mumbai.maticvigil.com";
let contract_address = "0x9e5c6b3d16411736c068026fc212e0b413dce243";
let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|| "default_value".to_string());

pub async fn save_certificate(sig_hash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let transport = Http::new(&node_url)?;
    let web3 = web3::Web3::new(transport);
    let wallet = web3::ethabi::Account::from_key(&private_key)?;
    let contract = Contract::new(&web3, *contract_address);

    let estimated_gas = contract.estimate_gas("storeHash", vec![sig_hash.as_bytes().to_vec()]).await?;
    let tx = Transaction::new()
        .gas_price(estimated_gas);

    let call=contract.call("storeHash", vec![sig_hash.as_bytes().to_vec()]);
    let signed_tx = wallet.sign_transaction(tx, &call).await?;
    let tx_hash = web3.eth().send_transaction(signed_tx).await?;
    Ok(tx_hash)
}