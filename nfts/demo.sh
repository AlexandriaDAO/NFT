#!/usr/bin/env bash
make clean
make build

dfx stop
dfx start --background --clean
dfx deploy --argument 'record {symbol="TT"; name="Testing Token";}' icrc7

dfx identity new alice --storage-mode=plaintext || true
dfx identity new bob --storage-mode=plaintext || true
YOU=$(dfx identity get-principal)
ALICE=$(dfx --identity alice identity get-principal)
BOB=$(dfx --identity bob identity get-principal)

echo '(*) Set managers:'
dfx canister call icrc7 set_managers \
    "vec{ principal\"$YOU\"}"

echo '(*) Set minters:'
dfx canister call icrc7 set_minters \
    "vec{ principal\"$YOU\"}"

echo '(*) Creating Token:'
dfx canister call icrc7 create_token \
    "(record{
        name=\"test\";
        asset_name=\"test\";
        asset_content_type=\"test\";
        asset_content=vec{};
        author=principal\"$YOU\";
        metadata=vec{}
     })"

echo '(*) Metadata of the newly created Token:'
dfx canister call icrc7 icrc7_token_metadata \
    "(vec{1})"

echo "(*) Owner of newly created Token:"
dfx canister call icrc7 icrc7_owner_of \
    "(vec{1})"

echo '(*) Mint newly created Token:'
dfx canister call icrc7 mint \
    "(record{
        token_id=1;
        holders=vec{principal\"$YOU\"}
    })"

echo '(*) Update Token:'
dfx canister call icrc7 update_token \
    "(record{
        id=1;
        name=opt \"updated_test\"
     })"

echo '(*) Metadata of the newly created Token:'
dfx canister call icrc7 icrc7_token_metadata \
    "(vec{1})"

echo "(*) Owner of newly created Token:"
dfx canister call icrc7 icrc7_owner_of \
    "(vec{0})"

echo "(*) Transfer newly created Token to alice:"
dfx canister call icrc7 icrc7_transfer \
    "(vec{record{
        to=record{owner=principal\"$ALICE\"};
        token_id=0
    }})"

echo "(*) Owner of newly created Token:"
dfx canister call icrc7 icrc7_owner_of \
    "(vec{0})"
