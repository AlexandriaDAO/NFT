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
    let collection_name = format!("{}Collection", quote! {#ident});
    let symbol = opts.symbol.clone();
    let name = opts.name.clone();
    let description = opts.description.clone();
    let logo = opts.logo.clone();
    let output = quote! {
        #[derive(Clone, Default, Deserialize, Serialize)]
        pub struct #collection_name{
            pub created_at: u64,
            pub updated_at: u64,
            pub minters: BTreeSet<Principal>,
            pub managers: BTreeSet<Principal>,
            pub settings: Settings,
        }

        impl Icrc7 for #ident {
            fn icrc7_collection_metadata(&self) -> serde_json::Value{
                serde_json::json!({
                })
            }
            fn symbol(&self) -> Result<&str>{
                Ok(#symbol)
            }
            fn name(&self) -> Result<&str>{
                Ok(#name)
            }
            fn description(&self) -> Result<Option<&str>>{
                Ok(#description)
            }
            fn logo(&self) -> Result<Option<&str>>{
                Ok(#logo)
            }
            // fn total_supply(&self) -> Result<usize>;
            // fn supply_cap(&self) -> Result<Option<usize>>;
            // fn max_query_batch_size(&self) -> Result<Option<usize>>;
            // fn max_update_batch_size(&self) -> Result<Option<usize>>;
            // fn default_take_value(&self) -> Result<Option<usize>>;
            // fn max_take_value(&self) -> Result<Option<usize>>;
            // fn max_memo_size(&self) -> Result<Option<usize>>;
            // fn atomic_batch_transfers(&self) -> Result<Option<bool>>;
            // fn tx_window(&self) -> Result<Option<usize>>;
            // fn permitted_drift(&self) -> Result<Option<usize>>;
            // fn token_metadata(&self, token_ids: Vec<usize>) -> Result<Vec<Option<serde_json::Value>>>;
            // fn owner_of(&self, token_ids: Vec<usize>) -> Result<Vec<Option<Account>>>;
            // fn balance_of(&self, accounts: Vec<Account>) -> Result<Vec<usize>>;
            // fn tokens(&self, prev: Option<usize>, take: Option<usize>) -> Result<Vec<usize>>;
            // fn tokens_of(
            //     &self,
            //     account: Account,
            //     prev: Option<usize>,
            //     take: Option<usize>,
            // ) -> Result<Vec<usize>>;
            // fn create_token(&mut self, args: CreateTokenArg) -> Result<Nat>;
            // fn update_token(args: UpdateTokenArg) -> Result<()>;
            // fn mint(&mut self, args: MintArg) -> Result<MintResult>;
            // fn transfer(&mut self, args: Vec<TransferArg>) -> Result<Vec<Option<TransferResult>>>;
        }
    };
    output.into()
}
