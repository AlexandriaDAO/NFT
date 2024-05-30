use candid::{CandidType, Nat, Principal};
use icp_token::Icrc7;

#[derive(Icrc7)]
#[icrc7(symbol = "TT")]
#[icrc7(name = "Test Token")]
pub struct Token {
    pub name: String,
    pub description: Option<String>,
    pub asset_name: String,
    pub asset_content_type: String,
    pub asset_hash: [u8; 32],
    pub author: Principal,
    //    pub supply_cap: Option<u32>,
    //    pub total_supply: u32,
    //    pub created_at: u64,
    //    pub updated_at: u64,
}

impl Token {
    pub fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}
