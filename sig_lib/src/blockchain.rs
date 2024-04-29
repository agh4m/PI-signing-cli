use futures::executor::block_on;
use serde_json::{from_str, Result};
use std::fs::read_to_string;
use web3::contract::{Contract, Options};
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
) -> Result<String> {
    let contract_address: Address = contract_address.parse().unwrap();
    let private_key: SecretKey = private_key.parse().unwrap();
    let wallet_address: Address = wallet_address.parse().unwrap();

    let abi_json = read_to_string("./abi.json").expect("Unable to read abi file");
    let contract_abi = from_str(&abi_json)?;

    let transport = Http::new(&node_url).unwrap();
    let web3 = Web3::new(transport);

    let contract = Contract::new(web3.eth(), contract_address, contract_abi);
    let call = contract
        .call(
            "storeHash",
            vec![sig_hash.as_bytes().to_vec()],
            wallet_address,
            Options::default(),
        )
        .await;
    let tx = TransactionParameters {
        to: Some(contract_address),
        gas: 100000.into(),
        data: call.unwrap().as_bytes().into(),
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

#[cfg(test)]
mod tests {
    use crate::blockchain::save_certificate;

    #[test]
    fn test_save_certificate() {
        todo!();
    }
}
