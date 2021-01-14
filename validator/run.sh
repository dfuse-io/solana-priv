#!/bin/bash
export RUST_LOG=solana=info,solana_metrics=error
export RUST_BACKTRACE=full

#echo "Cleaning up..."
#pushd "./ledger" &> /dev/null
#  ls | grep -v genesis.bin | xargs rm -r
#popd &> /dev/null

echo "Starting node..."
../target/release/solana-validator \
    --log - \
    --ledger ./ledger \
    --no-voting \
    --trusted-validator 2xte5CBkCBEBLNviyAfvSaTkMy6tvg99Cy3XJj9EJJs2 \
    --trusted-validator 3Ec6j5SkCj51PgH2fBUcXc4ptrwi6i5WpnCxZBak3cX3 \
    --trusted-validator 3KNGMiXwhy2CAWVNpLoUt25sNngFnX1mZpaiEeVccBA6 \
    --trusted-validator 3LboiLyZ3U1556ZBnNi9384C8Gz1LxFmzRnAojumnCJB \
    --trusted-validator 3RbsAuNknCTXuLyqmasnvYRpQg3MfWZ5N7WTi7ZGqdms \
    --trusted-validator 4TJZp9Ho82FrcRcBQes5oD52Y3QYeCxkpqWmjxmySQFY \
    --trusted-validator 5i6FL92EgjMmtFRogXeB7TaCYYAwd7kTQ9abKc1RTRMf \
    --trusted-validator 6GRLDLiAtx8ZjYgQgPo7UsYeJ9g1pLX5j3HK97tFmtXb \
    --trusted-validator 6cgsK8ph5tNUCiKG5WXLMZFX1CoL4jzuVouTPBwPC8fk \
    --trusted-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2 \
    --trusted-validator 7XSCAfoJ11zrQxonjbGZHLUL8tqpF7yhkxiieLds9mdH \
    --trusted-validator 8DM7JdDPShN4TFrWTwokvWmnCDqCy1D6VVLzSMDKri5V \
    --trusted-validator 8DXdM93UpEfqXezv1QTPhuA7Rci8MZujhsXQHoAsx5cN \
    --trusted-validator 9EBnt7k6Z4mRCiFMCN1kT8joN3SWHCuhQrW1a8M8mYPM \
    --trusted-validator 9hdNfC1DKGXxyqbNRSsStiPYoUREoRWZAEhmiqPw93yP \
    --trusted-validator 9rVx9wo6d3Akaq9YBw4sFuwc9oFGtzs8GsTfkHE7EH6t \
    --trusted-validator AUa3iN7h4c3oSrtP5pmbRcXJv8QSo4HGHPqXT4WnHDnp \
    --trusted-validator AaDBW2BYPC1cpCM6bYf5hN9MCNFz79fMHbK7VLXwrW5x \
    --trusted-validator AqGAaaACTDNGrVNVoiyCGiMZe8pcM1YjGUcUdVwgUtud \
    --trusted-validator BAbRkBYDhThcR8rn7wYtPYSxDnUCfRYx5dAjsuianuA6 \
    --trusted-validator Bb4BP3EvsPyBuqSAABx7KmYAp3mRqAZUYN1vChWsbjDc \
    --trusted-validator CVAAQGA8GBzKi4kLdmpDuJnpkSik6PMWSvRk3RDds9K8 \
    --trusted-validator CakcnaRDHka2gXyfbEd2d3xsvkJkqsLw2akB3zsN1D2S \
    --trusted-validator DE1bawNcRJB9rVm3buyMVfr8mBEoyyu73NBovf2oXJsJ \
    --trusted-validator DE37cgN2bGR26a1yQPPY42CozC1wXwXnTXDyyURHRsm7 \
    --trusted-validator F3LudCbGqu4DMqjduLq5WE2g3USYcjmVK3WR8KeNBhWz \
    --trusted-validator FVsjR8faKFZSisBatLNVo5bSH1jvHz3JvneVbyVTiV9K \
    --trusted-validator GdnSyH3YtwcxFvQrVVJMm1JhTS4QVX7MFsX56uJLUfiZ \
    --trusted-validator GosJ8GHbSUunTQPY5xEyjhY2Eg5a9qSuPhNC4Ctztr7y \
    --trusted-validator HoMBSLMokd6BUVDT4iGw21Tnxvp2G49MApewzGJr4rfe \
    --trusted-validator HzrEstnLfzsijhaD6z5frkSE2vWZEH5EUfn3bU9swo1f \
    --trusted-validator HzvGtvXFzMeJwNYcUu5pw8yyRxF2tLEvDSSFsAEBcBK2 \
    --trusted-validator J4B32eD2PmwCZyre5SWQS1jCU4NkiGGYLNapg9f8Dkqo \
    --trusted-validator ba2eZEU27TqR1MB9WUPJ2F7dcTrNsgdx38tBg53GexZ \
    --trusted-validator ba3zMkMp87HZg27Z7EDEkxE48zcKgJ59weFYtrKadY7 \
    --trusted-validator ba5rfuZ37gxhrLcsgA5fzCg8BvSQcTERPqY14Qffa3J \
    --trusted-validator tEBPZWSAdpzQoVzWBFD2qVGmZ7vB3Mh1Jq4tGZBx5eA \
    --deepmind \
    --rpc-port 8899 \
    --skip-poh-verify \
    --dynamic-port-range 8000-8010 \
    --entrypoint mainnet-beta.solana.com:8001 \
    --expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d \
    --wal-recovery-mode skip_any_corrupted_record \
    --limit-ledger-size \
    --gossip-host 173.177.17.65 \
    --private-rpc \
    --no-poh-speed-test \
    --no-port-check \
    --no-genesis-fetch \
#    --expected-shred-version=13490 \
#    --no-snapshot-fetch \
#    --no-untrusted-rpc \
#    --cuda \
#    --enable-rpc-transaction-history \