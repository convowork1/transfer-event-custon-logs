use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    json_types::U128, log, serde::Deserialize, serde::Serialize, serde_json, AccountId,
};
use serde_json::json;

// Test line here

pub const STANDARD: &str = "nep297";
pub const VERSION: &str = "1.0.0";

pub type EthAddress = [u8; 20];

#[derive(Default, BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub log_index: u64,
    pub log_entry_data: Vec<u8>,
    pub receipt_index: u64,
    pub receipt_data: Vec<u8>,
    pub header_data: Vec<u8>,
    pub proof: Vec<Vec<u8>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferDataEthereum {
    token: EthAddress,
    amount: U128,
}

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferDataNear {
    pub token: AccountId,
    pub amount: U128,
}

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub enum Event<'a> {
    SpectreBridgeNonceEvent {
        nonce: &'a U128,
        account: &'a AccountId,
        transfer: &'a TransferDataEthereum,
        recipient: &'a EthAddress,
    },
    SpectreBridgeTransferEvent {
        nonce: &'a U128,
        valid_till: u64,
        transfer: &'a TransferDataNear,
        fee: &'a TransferDataNear,
        recipient: &'a EthAddress,
    },
    SpectreBridgeTransferFailedEvent {
        nonce: &'a U128,
        account: &'a AccountId,
    },
    SpectreBridgeUnlockEvent {
        nonce: &'a U128,
        account: &'a AccountId,
    },
    SpectreBridgeDepositEvent {
        account: &'a AccountId,
        token: &'a AccountId,
        amount: &'a U128,
    },
    SpectreBridgeEthProoverNotProofedEvent {
        sender: &'a String,
        nonce: &'a U128,
        proof: &'a Proof,
    },
}

#[allow(dead_code)]
pub fn get_eth_address(address: String) -> EthAddress {
    let data = hex::decode(address).expect("address should be a valid hex string.");
    assert_eq!(data.len(), 20, "address should be 20 bytes long");
    let mut result = [0u8; 20];
    result.copy_from_slice(&data);
    result
}

impl Event<'_> {
    #[allow(dead_code)]
    pub fn emit(&self) {
        emit_event(&self);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventMessage {



    pub standard: String,
    pub version: String,
    pub event: serde_json::Value,
    pub data: serde_json::Value,
}

#[allow(dead_code)]
pub(crate) fn emit_event<T: ?Sized + Serialize>(data: &T) {
    let result = json!(data);
    let event_json = json!(EventMessage {
        standard: STANDARD.to_string(),
        version: VERSION.to_string(),
        event: result["event"].clone(),
        data: result["data"].clone(),
    })
    .to_string();
    log!(format!("EVENT_JSON:{}", event_json));
}

#[cfg(test)]
mod tests {
    fn test() {}
}
