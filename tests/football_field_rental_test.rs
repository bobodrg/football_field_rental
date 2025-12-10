use multiversx_sc_scenario::imports::*;
use std::fs;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

	blockchain.set_current_dir_from_workspace(".");

    blockchain.register_contract(
    	"mxsc:output/football-field-rental.mxsc.json",
    	football_field_rental::ContractBuilder,
	);



    blockchain
}

#[test]
fn run_all_scenarios() {
    let scenarios_dir = "scenarios";

    for entry in fs::read_dir(scenarios_dir).unwrap() {
        let path = entry.unwrap().path();

        if path.extension().unwrap() == "json" {
            println!("Running scenario: {}", path.display());
            world().run(path.to_str().unwrap());
        }
    }
}

