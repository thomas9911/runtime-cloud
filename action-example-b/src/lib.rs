wit_bindgen::generate!({ generate_all });
use exports::bettyblocks::runtime_cloud::action::Guest;

struct Action;

impl Guest for Action {
    fn execute() -> String {
        let str = format!("action b");
        str
    }
}

export!(Action);
