use icp_token::{Icrc7, Storage};
use serde::{Deserialize, Serialize};

#[derive(icp_token::candid::CandidType, Clone, Hash, Default, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub description: Option<String>,
}

impl icp_token::Metadata for Token {
    fn metadata(&self) -> String {
        serde_json::to_string(&serde_json::json!({
            "name": self.name,
            "description": self.description
        }))
        .unwrap_or_default()
    }
}

#[derive(Icrc7, Storage, Deserialize, Serialize, Default)]
#[icrc7(token_type = "Token")]
#[icrc7(symbol = "TT")]
#[icrc7(name = "Test Token")]
pub struct TokenCollections {}
