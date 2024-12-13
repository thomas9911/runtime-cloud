wit_bindgen::generate!({ generate_all });
use exports::bettyblocks::runtime_cloud::action_runner::Guest;

struct ActionRunner;

impl Guest for ActionRunner {
    fn execute() -> String {
        let yourinterface = wasmcloud::bus::lattice::CallTargetInterface::new(
            "bettyblocks",
            "runtime-cloud",
            "action",
        );

        wasmcloud::bus::lattice::set_link_name("action-example-b", vec![yourinterface]);

        // Calls over link foo to perform a keyvalue operation
        let x = bettyblocks::runtime_cloud::action::execute();

        let str = format!("henkie app:{}", x);
        str
    }
}

export!(ActionRunner);
