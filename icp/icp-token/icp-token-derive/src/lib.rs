use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(icrc7), forward_attrs(allow, doc, cfg))]
struct Opts {
    symbol: String,
    name: String,
    description: Option<String>,
    logo: Option<String>,
    assets_origin: Option<String>,
    total_supply: Option<u64>,
    supply_cap: Option<u64>,
    max_query_batch_size: Option<usize>,
    max_update_batch_size: Option<usize>,
    default_take_value: Option<usize>,
    max_take_value: Option<usize>,
    max_memo_size: Option<usize>,
    atomic_batch_transfers: Option<bool>,
    tx_window: Option<i64>,
    permitted_drift: Option<i64>,
    mutable: Option<bool>,
}

#[proc_macro_derive(Icrc7, attributes(icrc7))]
pub fn derive_icrc7(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let collection_name: proc_macro2::TokenStream =
        format!("{}Collection", quote! {#ident}).parse().unwrap();
    let symbol = opts.symbol.clone();
    let name = opts.name.clone();
    let description = match opts.description {
        Some(d) => d.clone(),
        None => "".to_string(),
    };
    let logo = match opts.logo {
        Some(d) => d.clone(),
        None => "".to_string(),
    };
    let supply_cap = match opts.supply_cap {
        Some(x) => quote! {
            fn supply_cap(&self) -> icp_token::Result<Option<usize>>{
                Ok(Some(#x))
            }
        },
        None => quote! {},
    };
    let max_query_batch_size = match opts.max_query_batch_size {
        Some(x) => quote! {
            fn max_query_batch_size(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let max_update_batch_size = match opts.max_update_batch_size {
        Some(x) => quote! {
            fn max_update_batch_size(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let default_take_value = match opts.default_take_value {
        Some(x) => quote! {
            fn default_take_value(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let max_take_value = match opts.max_take_value {
        Some(x) => quote! {
            fn max_take_value(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let max_memo_size = match opts.max_memo_size {
        Some(x) => quote! {
            fn max_memo_size(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let atomic_batch_transfers = match opts.atomic_batch_transfers {
        Some(x) => quote! {
            fn atomic_batch_transfers(&self) -> icp_token::Result<bool>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let tx_window = match opts.tx_window {
        Some(x) => quote! {
            fn tx_window(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let permitted_drift = match opts.permitted_drift {
        Some(x) => quote! {
            fn permitted_drift(&self) -> icp_token::Result<usize>{
                Ok(#x)
            }
        },
        None => quote! {},
    };
    let output = quote! {
        #[derive(Default)]
        pub struct #collection_name{
            pub total_supply: usize,
            pub created_at: u64,
            pub updated_at: u64,
            pub tokens: std::collections::HashMap<i64, #ident>,
            pub owners: std::collections::HashMap<Principal, std::collections::HashSet<i64>>,
        }

        impl icp_token::Icrc7Trait for #collection_name {
            fn icrc7_collection_metadata(&self) -> serde_json::Value{
                serde_json::json!({
                    "icrc7:symbol": self.symbol().unwrap(),
                    "icrc7:name": self.name().unwrap(),
                    "icrc7:description": self.description().unwrap(),
                    "icrc7:logo": self.logo().unwrap(),
                    "icrc7:total_supply": self.total_supply().unwrap(),
                    "icrc7:supply_cap": self.supply_cap().unwrap(),
                })
            }
            fn symbol(&self) -> icp_token::Result<&str>{
                Ok(#symbol)
            }
            fn name(&self) -> icp_token::Result<&str>{
                Ok(#name)
            }
            fn description(&self) -> icp_token::Result<&str>{
                Ok(#description)
            }
            fn logo(&self) -> icp_token::Result<&str>{
                Ok(#logo)
            }
            fn total_supply(&self) -> icp_token::Result<usize>{
                Ok(self.total_supply)
            }
            #supply_cap
            #max_query_batch_size
            #max_update_batch_size
            #default_take_value
            #max_take_value
            #max_memo_size
            #atomic_batch_transfers
            #tx_window
            #permitted_drift
            fn token_metadata(&self, token_ids: Vec<i64>) -> icp_token::Result<Vec<serde_json::Value>>{
                if token_ids.len() > self.max_query_batch_size()?{
                    Err(icp_token::Error::Custom("exceeds max query batch size"))
                } else {
                    Ok(self.tokens.iter().filter(|(id, _)| token_ids.contains(id)).map(|(id, token)| token.metadata()).collect())
                }
            }
            fn owner_of(&self, token_ids: Vec<i64>) -> icp_token::Result<Vec<Option<Principal>>>{
                if token_ids.len() > self.max_query_batch_size()?{
                    Err(icp_token::Error::Custom("exceeds max query batch size"))
                } else {
                    Ok(token_ids.iter().map(|id| self.owners.iter().filter(|(_, ids)| ids.contains(id)).next()).map(|v| if let Some((o, _)) = v {Some(o.clone())} else {None}).collect())
                }
            }
            fn balance_of(&self, accounts: Vec<Principal>) -> icp_token::Result<Vec<usize>>{
                if accounts.len() > self.max_query_batch_size()?{
                    Err(icp_token::Error::Custom("exceeds max query batch size"))
                } else {
                    Ok(accounts.iter().map(|owner| if let Some(tt) = self.owners.get(owner){ tt.len()}else{0}).collect())
                }
            }
            fn tokens(&self, prev: Option<usize>, take: Option<usize>) -> icp_token::Result<Vec<i64>>{
                Ok(self.tokens.iter().skip(prev.unwrap_or(0)).take(take.unwrap_or(self.max_query_batch_size()?)).map(|(id, _)| id.clone()).collect())
            }
            fn tokens_of(
                &self,
                account: Principal,
                prev: Option<usize>,
                take: Option<usize>,
            ) -> icp_token::Result<Vec<i64>>{
                if let Some(tt) = self.owners.get(&account){
                    Ok(tt.iter().skip(prev.unwrap_or(0)).take(take.unwrap_or(self.max_query_batch_size()?)).map(|s| s.clone()).collect())
                } else {
                    Ok(vec![])
                }
            }
            // fn create_token(&mut self, args: CreateTokenArg) -> Result<Nat>;
            // fn update_token(args: UpdateTokenArg) -> Result<()>;
            // fn mint(&mut self, args: MintArg) -> Result<MintResult>;
            // fn transfer(&mut self, args: Vec<TransferArg>) -> Result<Vec<Option<TransferResult>>>;
        }
    };
    output.into()
}
