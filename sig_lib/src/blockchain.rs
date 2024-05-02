use ethabi::{Contract, Token};
use futures::executor::block_on;
use std::error::Error;
use std::fs::File;
use std::result::Result;
use web3::signing::SecretKey;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters};
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

    let mut sig_hash_bytes = sig_hash.as_bytes().to_vec();
    sig_hash_bytes.resize(32, 0);
    let arg: Token = Token::FixedBytes(sig_hash_bytes);

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

    return Ok(tx_hash.to_string());
}
