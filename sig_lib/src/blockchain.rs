use std::error::Error;
use web3::contract::Contract;
use web3::transports::Http;
use web3::types::Transaction;
use web3::{ethabi, Web3};

pub async fn save_certificate(sig_hash: &str, private_key: &str) -> Result<String, Box<dyn Error>> {
    let node_url = "https://rpc-mumbai.maticvigil.com";
    let contract_address = "0x9e5c6b3d16411736c068026fc212e0b413dce243";

    let transport = Http::new(&node_url)?;
    let web3 = Web3::new(transport);
    let wallet = ethabi::Account::from_key(&private_key)?;

    let contract = Contract::new(&web3, *contract_address);

    let estimated_gas = contract
        .estimate_gas("storeHash", vec![sig_hash.as_bytes().to_vec()])
        .await?;
    let tx = Transaction::new().gas_price(estimated_gas);

    let call = contract.call("storeHash", vec![sig_hash.as_bytes().to_vec()]);
    let signed_tx = wallet.sign_transaction(tx, &call).await?;
    let tx_hash = web3.eth().send_transaction(signed_tx).await?;

    return Ok(tx_hash.to_string());
}

