#!/bin/bash

dfx identity use minter
MINTER_PRINCIPAL=$(dfx identity get-principal)
MINTER_ACCOUNT_ID=$(dfx ledger account-id)

dfx identity use default
DEFAULT_PRINCIPAL=$(dfx identity get-principal)
DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

dfx deploy GLYPH --argument '
(variant {
Init = record {
token_name = "GLYPHs";
token_symbol = "GLYPH";
minting_account = record {
owner = principal "'$DEFAULT_PRINCIPAL'";
};
initial_balances = vec {
record {
record {
owner = principal "'$MINTER_PRINCIPAL'";
};
100_000_000_000;
};
};
metadata = vec {};
transfer_fee = 10_000;
archive_options = record {
trigger_threshold = 2000;
num_blocks_to_archive = 1000;
controller_id = principal "'$DEFAULT_PRINCIPAL'";
};
feature_flags = opt record {
icrc2 = true;
};
}
})
'

dfx deploy --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister --argument "
(variant {
Init = record {
minting_account = \"$MINTER_ACCOUNT_ID\";
initial_values = vec {
record {
\"$DEFAULT_ACCOUNT_ID\";
record {
e8s = 10_000_000_000 : nat64;
};
};
};
send_whitelist = vec {};
transfer_fee = opt record {
e8s = 10_000 : nat64;
};
token_symbol = opt \"LICP\";
token_name = opt \"Local ICP\";
}
})
"

dfx deploy