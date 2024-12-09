wit_bindgen::generate!({ generate_all });
use exports::bettyblocks::runtime_cloud::action_runner::Guest;

struct ActionRunner;

impl Guest for ActionRunner {
    fn execute() -> String {
        String::from("harm")
    }
}

export!(ActionRunner);