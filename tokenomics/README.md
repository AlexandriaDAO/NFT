# Tokenomics

This Repo has 3 DeFI canister. 

One holds and distributes ICP-based Revenue collected on UncensoredGreats

Another Distributes GLYPHs (user-credits) to people that want to pay for on-chain services.

Another (TBD) authomates the distribution of ICP and GLYPHs.

## Set up local ledger canister.

1. Configure Defualt & Minter IDs

- dfx identity use minter
- export MINTER=$(dfx identity get-principal)
- export MINTER_ACCOUNT_ID=$(dfx ledger account-id)

- dfx identity use default
- export DEFAULT=$(dfx identity get-principal)
- export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

2. DEPLOY THE LEDGER CANISTER (locally)

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
3. CALL IT TO SAY HELLO / CHECK BALANCES

- dfx canister call icp_ledger_canister name

- dfx canister call icp_ledger_canister account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'

4. Deploy and Intialize ICP Transfer Canister

- dfx deploy icp_transfer_backend

- export TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister icp_transfer_backend)"

- export TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')"

5. Transfer ICP and Test Transfer Canister

dfx canister call icp_ledger_canister transfer "(record { to = ${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })"

dfx canister call icp_transfer_backend transfer "(record { amount = record { e8s = 1_00_000_000 }; to_principal = principal \"$(dfx identity get-principal)\"})"


## Depoy the ICRC Token. 

Details about customizability here: https://internetcomputer.org/docs/current/developer-docs/defi/icrc-1/icrc1-ledger-setup

We call ours GLYPH.

dfx deploy GLYPH --specified-id hdtfn-naaaa-aaaam-aciva-cai --argument '
  (variant {
    Init = record {
      token_name = "GLYPHs";
      token_symbol = "GLYPH";
      minting_account = record {
        owner = principal "'${DEFAULT}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${MINTER}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10_000;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${DEFAULT}'";
      };
      feature_flags = opt record {
        icrc2 = true;
      };
    }
  })
'

## Deploy & test the ICRC Transfering Canister.

- dfx deploy token_transfer_backend --specified-id ju4sh-3yaaa-aaaap-ahapa-cai

Might have to adjust these for ICRC2:

- dfx canister call GLYPH icrc1_transfer "(record {
  to = record {
    owner = principal \"$(dfx canister id token_transfer_backend)\";
  };
  amount = 1_000_000_000;
})"


- dfx canister call token_transfer_backend transfer "(record {
  amount = 100_000_000;
  to_account = record {
    owner = principal \"$(dfx identity get-principal)\";
  };
})"

Check the balances of each (*Note, tokens sent to the minting account are burned and do not appear in balance): 

- dfx canister call GLYPH icrc1_balance_of '(record { owner = principal "ju4sh-3yaaa-aaaap-ahapa-cai" })'
(2_800_000_000 : nat)

- dfx canister call GLYPH icrc1_balance_of '(record { owner = principal "mxtax-xmovu-wu5th-gdf4k-vfkdn-ffsxn-e67ju-sidls-4dr2i-3mqoe-tae" })'