#!/bin/bash

export RUST_LOG=solana=info,solana_metrics=error

../target/debug/solana-validator \
    --log - \
    --ledger "$SOLANA_LEDGER_DATA_DIR" \
    --no-voting \
    --trusted-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2 \
    --trusted-validator GdnSyH3YtwcxFvQrVVJMm1JhTS4QVX7MFsX56uJLUfiZ \
    --trusted-validator DE1bawNcRJB9rVm3buyMVfr8mBEoyyu73NBovf2oXJsJ \
    --trusted-validator CakcnaRDHka2gXyfbEd2d3xsvkJkqsLw2akB3zsN1D2S \
    --trusted-validator tEBPZWSAdpzQoVzWBFD2qVGmZ7vB3Mh1Jq4tGZBx5eA \
    --trusted-validator CVAAQGA8GBzKi4kLdmpDuJnpkSik6PMWSvRk3RDds9K8 \
    --trusted-validator ba2eZEU27TqR1MB9WUPJ2F7dcTrNsgdx38tBg53GexZ \
    --trusted-validator HzvGtvXFzMeJwNYcUu5pw8yyRxF2tLEvDSSFsAEBcBK2 \
    --trusted-validator J4B32eD2PmwCZyre5SWQS1jCU4NkiGGYLNapg9f8Dkqo \
    --trusted-validator 4TJZp9Ho82FrcRcBQes5oD52Y3QYeCxkpqWmjxmySQFY \
    --trusted-validator GosJ8GHbSUunTQPY5xEyjhY2Eg5a9qSuPhNC4Ctztr7y \
    --trusted-validator 6cgsK8ph5tNUCiKG5WXLMZFX1CoL4jzuVouTPBwPC8fk \
    --rpc-port 8899 \
    --private-rpc \
    --dynamic-port-range 8000-8010 \
    --entrypoint mainnet-beta.solana.com:8001 \
    --expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d \
    --wal-recovery-mode skip_any_corrupted_record \
    --limit-ledger-size \
    --no-snapshot-fetch \
    --gossip-host "$GOSSIP_HOST" \
#    --no-port-check \
#    --expected-shred-version 13490 \
#    --no-untrusted-rpc \
#    --cuda \
#    --enable-rpc-transaction-history \
