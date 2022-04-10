use wallet::bitcoin::util::base58;
use bitcoin_hashes::{Hash, hash160};
use std::convert::TryInto;
use std::str::FromStr;
use wallet::bitcoin::{OutPoint, Script, Transaction, TxIn, TxOut};
use wallet::bitcoin::blockdata::opcodes;
use wallet::bitcoin::hashes::sha256d;
use wallet::account::{Account, AccountAddressType, MasterAccount, Unlocker};
use wallet::mnemonic::Mnemonic;
use hex::decode;
use wallet::bitcoin::{Address, Network};

const PASSPHRASE: &str = "correct horse battery staple";

pub fn get_pubkey_hash(address: &str) -> [u8; 20] {
    base58::from_check(address).unwrap().try_into().unwrap()
}

pub fn get_hash_from_pubkey(pubkey: &str) -> [u8;20]{
    let hash = hash160::Hash::hash(&hex::decode(pubkey).unwrap());
    println!("hash: {:?}", hex::encode(hash.as_inner()));
    hash.as_inner().to_owned()
}

pub fn generate_address(index: u32, change:u32) -> Address {
    let words = dotenv!("WORDS");
    let mnemonic = Mnemonic::from_str(words).unwrap();
    let mut master = MasterAccount::from_mnemonic(&mnemonic, 0, Network::Testnet, PASSPHRASE, None).unwrap();
    let mut unlocker = Unlocker::new_for_master(&master, PASSPHRASE).unwrap();
    let account = Account::new(&mut unlocker, AccountAddressType::P2PKH, 0, change, 10).unwrap();
    account.get_key(index).unwrap().address.clone()
}

pub fn create_tx() {
    /**
    prev_tx: https://blockstream.info/testnet/tx/8b61a11c9c331c49836fee725ed5ca3d35c7a504b375ffb23e5725fe22123d95
    **/
    let prev_txid="8b61a11c9c331c49836fee725ed5ca3d35c7a504b375ffb23e5725fe22123d95";
    let prev_index: u32=0;
    let from_address = generate_address(0,0);// m/44'/1'/0'/0/0 mj6CXnFA2Vd3FK7cBWoNz3LEzstsNJ2Sxf
    assert_eq!(from_address, Address::from_str(&"mj6CXnFA2Vd3FK7cBWoNz3LEzstsNJ2Sxf").unwrap());
    let receive_address = generate_address(1,0);
    assert_eq!(receive_address, Address::from_str(&"mzoPjx5YGkJdX4Trh6unS1JAHcwQsQ4qYX").unwrap());
    let receive_amount: u64 = 900;
    let receive_script_pubkey=receive_address.script_pubkey();
    assert_eq!(
        receive_address.script_pubkey(),
        Script::from_str("76a914d3864974ef7e80f9f77a313912e307a73366fe3588ac").unwrap()
    );
    let change_address = generate_address(0,1);
    assert_eq!(change_address, Address::from_str(&"mvfaMSQs8MDvDP3pUwsqfz4XZzTYfBErkV").unwrap());
    let change_amount: u64 = 100;
    let change_script_pubkey=change_address.script_pubkey();
    assert_eq!(
        change_address.script_pubkey(),
        Script::from_str("76a914a62b50801ab13942bbee2a09f1ab5dc5ff4b8ca388ac").unwrap()
    );
    let mut raw_tx = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![
            TxIn {
                previous_output: OutPoint::from_str("8b61a11c9c331c49836fee725ed5ca3d35c7a504b375ffb23e5725fe22123d95:0").unwrap(),
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF,
                witness: vec![]
            },
        ],
        output: vec![
            TxOut {
                value: receive_amount,
                script_pubkey: receive_script_pubkey,
            },
            TxOut {
                value: change_amount,
                script_pubkey: change_script_pubkey,
            },
        ],
    };
    let sig_hash = raw_tx.signature_hash(0, &from_address.script_pubkey(), 1);
}


