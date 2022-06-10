use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::{env, ext_contract, near_bindgen, Promise, PromiseResult};
use std::str::FromStr;

near_sdk::setup_alloc!();

// define the methods we'll use on the other contract
#[ext_contract(ext_ft)]
pub trait RainbowContract {
    fn migrate_to_ethereum(&mut self, eth_recipient: String);
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn crosscontract_call_rust(&self, eth_recipient:String, amount :String) -> Promise {
        // Invoke a method on another contract
        // This will send an ActionReceipt to the shard where the contract lives.
        let amount= u128::from_str(&amount).unwrap();
        ext_ft::migrate_to_ethereum(
            eth_recipient.to_string(),
            &"enear.goerli.testnet", // contract account id
            amount, // yocto NEAR to attach
            50000000000000 // gas to attach
            
        )

        .then(ext_self::my_callback(
            &env::current_account_id(), // this contract's account id
            0, // yocto NEAR to attach to the callback
            50000000000000 // gas to attach to the callback
        ))
    }
    pub fn my_callback(&self) -> String {
        let mut ch="";
        if env::promise_results_count()==1 {
            ch="this is rust callback method"
        }
        return ch.to_string();
        
    }
}