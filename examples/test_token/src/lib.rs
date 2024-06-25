use serde::{Deserialize, Serialize};
use uncensored_greats_dao::{Icrc7, Metadata, Storage};

#[derive(
    uncensored_greats_dao::candid::CandidType, Clone, Hash, Default, Serialize, Deserialize,
)]
pub struct TestToken {
    pub name: String,
    pub description: Option<String>,
}

impl Metadata for TestToken {
    fn metadata(&self) -> String {
        serde_json::to_string(&serde_json::json!({
            "name": self.name,
            "description": self.description
        }))
        .unwrap_or_default()
    }
}

#[derive(Icrc7, Storage, Deserialize, Serialize, Default)]
#[icrc7(token_type = "TestToken")]
#[icrc7(symbol = "TT")]
#[icrc7(name = "Test Token")]
pub struct TokenCollections {}
