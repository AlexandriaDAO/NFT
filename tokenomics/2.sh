#!/bin/bash

TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister icp_transfer_backend)"

TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')"

echo Transfer ICP Funds to the Canister
dfx canister call icp_ledger_canister transfer "(record { to =
${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; memo = 1; amount = record { e8s =
2_00_000_000 }; fee = record { e8s = 10_000 }; })"

#echo Transfer GLYPH Funds to the Canister:
#dfx canister call GLYPH icrc2_transfer_from "(record { to =
#${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; memo = 1; amount = record { e8s =
#2_00_000_000 }; fee = record { e8s = 10_000 }; })"


echo MINT GLYPHs!
dfx canister call GLYPH icrc1_transfer '(
record { to = record {
owner = principal "ie5gv-y6hbb-ll73p-q66aj-4oyzt-tbcuh-odt6h-xkpl7-bwssd-lgzgw-5qe";
subaccount = opt vec {
0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0;
0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0;
};
};
from = record {
owner = principal "mxtax-xmovu-wu5th-gdf4k-vfkdn-ffsxn-e67ju-sidls-4dr2i-3mqoe-tae";
subaccount = opt vec {
0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0;
0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0; 0;
};
};
amount = 10000;
fee = null;
}
)'
