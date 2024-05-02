use ethabi::{Function, Token,Contract};
use futures::executor::block_on;
use serde_json::{from_str, Result};
use std::fs::read_to_string;
use web3::signing::SecretKey;
use web3::transports::Http;
use web3::types::{Address, Bytes, TransactionParameters, TransactionReceipt, H256};
use web3::Web3;
use std::fs::File;

pub async fn save_certificate(
    sig_hash: &str,
    node_url: &str,
    contract_address: &str,
    private_key: &str,
    wallet_address: &str,
) -> Result<String> {
    let contract_address: Address = contract_address.parse().unwrap();
    let private_key: SecretKey = private_key.parse().unwrap();
    let _wallet_address: Address = wallet_address.parse().unwrap();
    let transport = Http::new(&node_url).unwrap();
    let web3 = Web3::new(transport);

    let abi_file = File::open("./abi.json").unwrap();
    let contract = Contract::load(abi_file).unwrap();
    let function = contract.function("storeHash").unwrap();

    let mut sig_hash_bytes = sig_hash.as_bytes().to_vec();
    sig_hash_bytes.resize(32, 0);
    let arg: Token = Token::FixedBytes(sig_hash_bytes);

    let encoded_call = function.encode_input(&[arg]);

    let tx = TransactionParameters {
        to: Some(contract_address),
        gas: 100000.into(),
        data: encoded_call.unwrap().into(),
        chain_id: Some(80002),
        ..Default::default()
    };

    let signed_tx = block_on(web3.accounts().sign_transaction(tx, &private_key));
    let tx_hash = web3
        .eth()
        .send_raw_transaction(signed_tx.unwrap().raw_transaction)
        .await;

    return Ok(tx_hash.unwrap().to_string());
}

pub async fn get_certificate_hash(node_url: &str, tx_hash: &str) -> Result<String> {
    let transport = Http::new(&node_url).unwrap();
    let web3 = Web3::new(transport);

    let tx_hash: H256 = tx_hash.parse().unwrap();
    let receipt: TransactionReceipt = web3
        .eth()
        .transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap();
    let log = &receipt.logs[0];
    let stored_hash: &Bytes = &log.data;
    return Ok(format!("{:x?}", stored_hash));
}

