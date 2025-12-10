// #![allow(non_snake_case)]

// pub mod config;
// mod football_field_rental_proxy;
// use football_field_rental_proxy as proxy;


// use config::Config;
// use multiversx_sc_snippets::imports::*;
// use serde::{Deserialize, Serialize};
// use std::{
//     io::{Read, Write},
//     path::Path,
// };

// const STATE_FILE: &str = "state.toml";

// pub async fn football_field_rental_cli() {
//     env_logger::init();

//     let mut args = std::env::args();
//     let _ = args.next();
//     let cmd = args.next().expect("at least one argument required");
//     let config = Config::new();
//     let mut interact = ContractInteract::new(config).await;
//     match cmd.as_str() {
//         "deploy" => interact.deploy().await,
//         "createFootballSlot" => interact.create_football_slot().await,
//         "participateToFootballSlot" => interact.participate_to_football_slot().await,
//         "cancelFootballSlot" => interact.cancel_football_slot().await,
//         "setFootballFieldManager" => interact.set_football_field_manager().await,
//         "payCourt" => interact.pay_court().await,
//         "setFootballCourtCost" => interact.set_football_court_cost().await,
//         "confirmSlot" => interact.confirm_slot().await,
//         "getSlotStatus" => interact.get_slot_status().await,
//         _ => panic!("unknown command: {}", &cmd),
//     }
// }

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct State {
//     contract_address: Option<Bech32Address>
// }

// impl State {
//         // Deserializes state from file
//         pub fn load_state() -> Self {
//             if Path::new(STATE_FILE).exists() {
//                 let mut file = std::fs::File::open(STATE_FILE).unwrap();
//                 let mut content = String::new();
//                 file.read_to_string(&mut content).unwrap();
//                 toml::from_str(&content).unwrap()
//             } else {
//                 Self::default()
//             }
//         }
    
//         /// Sets the contract address
//         pub fn set_address(&mut self, address: Bech32Address) {
//             self.contract_address = Some(address);
//         }
    
//         /// Returns the contract address
//         pub fn current_address(&self) -> &Bech32Address {
//             self.contract_address
//                 .as_ref()
//                 .expect("no known contract, deploy first")
//         }
//     }
    
//     impl Drop for State {
//         // Serializes state to file
//         fn drop(&mut self) {
//             let mut file = std::fs::File::create(STATE_FILE).unwrap();
//             file.write_all(toml::to_string(self).unwrap().as_bytes())
//                 .unwrap();
//         }
//     }

// pub struct ContractInteract {
//     interactor: Interactor,
//     wallet_address: Address,
//     contract_code: BytesValue,
//     state: State
// }

// impl ContractInteract {
//     pub async fn new(config: Config) -> Self {
//         let mut interactor = Interactor::new(config.gateway_uri())
//             .await
//             .use_chain_simulator(config.use_chain_simulator());

//         interactor.set_current_dir_from_workspace("football-field-rental");
//         let wallet = Wallet::from_pem_file("wallets/bob.pem")
//     		.expect("Could not load PEM file");

// 		let wallet_address = interactor.register_wallet(wallet.clone()).await;


//         // Useful in the chain simulator setting
//         // generate blocks until ESDTSystemSCAddress is enabled
//         interactor.generate_blocks_until_all_activations().await;
        
//         let contract_code = BytesValue::interpret_from(
//             "mxsc:../output/football-field-rental.mxsc.json",
//             &InterpreterContext::default(),
//         );

//         ContractInteract {
//             interactor,
//             wallet_address,
//             contract_code,
//             state: State::load_state()
//         }
//     }

//     pub async fn deploy(&mut self) {
//         let manager = self.wallet_address.clone();
// 		let court_cost = BigUint::<StaticApi>::from(1_000_000_000_000_000u128); // 0.001 EGLD

//         let new_address = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .init(manager, court_cost)
//             .code(&self.contract_code)
//             .returns(ReturnsNewAddress)
//             .run()
//             .await;
//         let new_address_bech32 = new_address.to_bech32_default();
//         println!("new address: {new_address_bech32}");
//         self.state.set_address(new_address_bech32);
//     }

//     pub async fn create_football_slot(&mut self) {
// 		let egld_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);

// 		let start = 1000u64;
// 		let end = 2000u64;


//         let response = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .to(self.state.current_address())
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .create_football_slot(start, end)
//             .egld(egld_amount)
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {response:?}");
//     }

//     pub async fn participate_to_football_slot(&mut self) {
// 		let egld_amount = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);

//         let response = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .to(self.state.current_address())
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .participate_to_football_slot()
//             .egld(egld_amount)
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {response:?}");
//     }

//     pub async fn cancel_football_slot(&mut self) {
//         let response = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .to(self.state.current_address())
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .cancel_football_slot()
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {response:?}");
//     }

//     pub async fn set_football_field_manager(&mut self) {
//     	let new_manager = self.wallet_address.clone();

//     	let response = self
//         	.interactor
//         	.tx()
//         	.from(&self.wallet_address)
//         	.to(self.state.current_address())
//         	.gas(30_000_000u64)
//         	.typed(proxy::FootballFieldRentalProxy)
//         	.set_football_field_manager(new_manager)
//         	.returns(ReturnsResultUnmanaged)
//         	.run()
//         	.await;

//     	println!("Manager set successfully. Result: {response:?}");
// 	}

//     pub async fn pay_court(&mut self) {
//         let response = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .to(self.state.current_address())
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .pay_court()
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {response:?}");
//     }

//     pub async fn set_football_court_cost(&mut self) {
// 		let new_cost = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);

//         let response = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .to(self.state.current_address())
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .set_football_court_cost(new_cost)
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {response:?}");
//     }

//     pub async fn confirm_slot(&mut self) {
//         let response = self
//             .interactor
//             .tx()
//             .from(&self.wallet_address)
//             .to(self.state.current_address())
//             .gas(30_000_000u64)
//             .typed(proxy::FootballFieldRentalProxy)
//             .confirm_slot()
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {response:?}");
//     }

//     pub async fn get_slot_status(&mut self) {
//         let result_value = self
//             .interactor
//             .query()
//             .to(self.state.current_address())
//             .typed(proxy::FootballFieldRentalProxy)
//             .get_slot_status()
//             .returns(ReturnsResultUnmanaged)
//             .run()
//             .await;

//         println!("Result: {result_value:?}");
//     }

// }
#![allow(non_snake_case)]

pub mod config;
mod football_field_rental_proxy;
use football_field_rental_proxy as proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{io::{Read, Write}, path::Path};

const STATE_FILE: &str = "state.toml";

pub async fn football_field_rental_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;

    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "createFootballSlot" => interact.create_football_slot().await,
        "participateToFootballSlot" => interact.participate_to_football_slot().await,
        "confirmSlot" => interact.confirm_slot().await,
        "payCourt" => interact.pay_court().await,
        "setFootballCourtCost" => interact.set_football_court_cost().await,
        "getSlotStatus" => interact.get_slot_status().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes()).unwrap();
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    wallet_creator: Address,
    wallet_participant: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("football-field-rental");

        let wallet_creator = interactor.register_wallet(test_wallets::bob()).await;
        let wallet_participant = interactor.register_wallet(test_wallets::alice()).await;

        interactor.generate_blocks_until_all_activations().await;

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/football-field-rental.mxsc.json",
            &InterpreterContext::default(),
        );

        Self {
            interactor,
            wallet_creator,
            wallet_participant,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let court_cost = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_creator)
            .gas(30_000_000)
            .typed(proxy::FootballFieldRentalProxy)
            .init(self.wallet_creator.clone(), court_cost)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        let bech32 = new_address.to_bech32_default();
        println!("Deploy addr: {}", bech32);

        self.state.set_address(bech32);
    }

    pub async fn create_football_slot(&mut self) {
        let egld = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);
		let start = 1000u64;
		let end = 2000u64;
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_creator)
            .to(self.state.current_address())
            .typed(proxy::FootballFieldRentalProxy)
            .create_football_slot(start, end)
            .egld(egld)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Create slot: {:?}", result);
    }

    pub async fn participate_to_football_slot(&mut self) {
        let egld = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);

        let result = self
            .interactor
            .tx()
            .from(&self.wallet_participant)   // <-- ALICE, NU BOB
            .to(self.state.current_address())
            .typed(proxy::FootballFieldRentalProxy)
            .participate_to_football_slot()
            .egld(egld)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Participation: {:?}", result);
    }

    pub async fn confirm_slot(&mut self) {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_creator) // doar manager/owner
            .to(self.state.current_address())
            .typed(proxy::FootballFieldRentalProxy)
            .confirm_slot()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Confirm slot: {:?}", result);
    }

    pub async fn pay_court(&mut self) {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_creator)
            .to(self.state.current_address())
            .typed(proxy::FootballFieldRentalProxy)
            .pay_court()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Pay court: {:?}", result);
    }

    pub async fn set_football_court_cost(&mut self) {
        let cost = BigUint::<StaticApi>::from(1_000_000_000_000_000u128);

        let result = self
            .interactor
            .tx()
            .from(&self.wallet_creator)
            .to(self.state.current_address())
            .typed(proxy::FootballFieldRentalProxy)
            .set_football_court_cost(cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Set cost: {:?}", result);
    }

    pub async fn get_slot_status(&mut self) {
        let result = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FootballFieldRentalProxy)
            .get_slot_status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Slot status: {:?}", result);
    }
}
