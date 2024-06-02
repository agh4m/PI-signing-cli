use ethabi::{Contract, Token};
use futures::executor::block_on;
use std::error::Error;
use std::fs::File;
use std::result::Result;
use web3::signing::SecretKey;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, H256};
use web3::Web3;

pub async fn save_certificate(
    sig_hash: &str,
    node_url: &str,
    contract_address: &str,
    private_key: &str,
    wallet_address: &str,
) -> Result<String, Box<dyn Error>> {
    let contract_address: Address = contract_address.parse()?;
    let private_key: SecretKey = private_key.parse()?;
    let _wallet_address: Address = wallet_address.parse()?;
    let transport = Http::new(&node_url)?;
    let web3 = Web3::new(transport);

    let abi_file = File::open("./abi.json")?;
    let contract = Contract::load(abi_file)?;
    let function = contract.function("storeHash")?;

    let arg: Token = Token::String(sig_hash.to_string());

    let encoded_call = function.encode_input(&[arg])?;

    let tx = TransactionParameters {
        to: Some(contract_address),
        gas: 100000.into(),
        data: encoded_call.into(),
        chain_id: Some(80002),
        ..Default::default()
    };

    let signed_tx = block_on(web3.accounts().sign_transaction(tx, &private_key))?;
    let tx_hash = web3
        .eth()
        .send_raw_transaction(signed_tx.raw_transaction)
        .await?;

    let hash = tx_hash
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    return Ok(hash);
}

pub async fn get_certificate_hash(node_url: &str, tx_hash: &str) -> Result<String, Box<dyn Error>> {
    let transport = Http::new(&node_url)?;
    let web3 = Web3::new(transport);

    let tx_hash: H256 = tx_hash.parse()?;
    let receipt = web3.eth().transaction_receipt(tx_hash).await?;

    match receipt {
        Some(receipt) => {
            let log = &receipt.logs[0];
            let stored_hash = &log.data.0;
            let hash = stored_hash.iter().map(|b| *b as char).collect::<String>();
            return Ok(hash);
        }
        None => Err("Failed to get stored_hash")?,
    }
}
