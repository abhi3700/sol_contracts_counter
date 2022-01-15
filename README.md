# Counter
A Counter program for Solana Blockchain
## About
* Only the Admin can add value to existing `count` value.
* Anyone can increment the `count` value.

## System
Tested successfully with following versions for different OS.

### MacOS M1
```
rust: 1.57.0
solana-cli: 1.8.0
anchor: 0.20.1
```

## Build
* `$ yarn build`
```console
❯ yarn build
yarn run v1.22.17
$ anchor build
BPF SDK: /Users/abhi3700/.cargo/bin/sdk/bpf
cargo-build-bpf child: rustup toolchain list -v
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
   Compiling counter v0.1.0 (/Users/abhi3700/F/coding/github_repos/sol_contracts_counter/programs/counter)
    Finished release [optimized] target(s) in 1.61s
cargo-build-bpf child: /Users/abhi3700/.cargo/bin/sdk/bpf/scripts/strip.sh /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/bpfel-unknown-unknown/release/counter.so /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/deploy/counter.so
cargo-build-bpf child: /Users/abhi3700/.cargo/bin/sdk/bpf/dependencies/bpf-tools/llvm/bin/llvm-readelf --dyn-symbols /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/deploy/counter.so

To deploy this program:
  $ solana program deploy /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/deploy/counter.so
The program address will default to this keypair (override with --program-id):
  /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/deploy/counter-keypair.json
✨  Done in 2.21s.
```

## Deploy
Follow this before deploying the program:

1. Copy the program_id from `$ yarn pgid`
2. Paste into `declare_id` param of `lib.rs` file.
3. Paste into `Anchor.toml` file at <program_name> param for different networks like localnet, devnet, mainnet.
4. For all, the keypair used is saved at this location: "~/.config/solana/id.json".

### Localnet
### Devnet
* 1st time deployment via `$ yarn deploydev`
```console
❯ yarn deploydev
yarn run v1.22.17
$ anchor deploy --provider.cluster devnet
Deploying workspace: https://api.devnet.solana.com
Upgrade authority: /Users/abhi3700/.config/solana/id.json
Deploying program "counter"...
Program path: /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/deploy/counter.so...
Program Id: 9b1LijhFmtMDfQAuq9aRVqRiFLeG6Edm5JGiDHZu3AUM

Deploy success
✨  Done in 17.81s.
```
* After upgrade via `$ yarn deploydev`
```console
❯ yarn deploydev
yarn run v1.22.17
$ anchor deploy --provider.cluster devnet
Deploying workspace: https://api.devnet.solana.com
Upgrade authority: /Users/abhi3700/.config/solana/id.json
Deploying program "counter"...
Program path: /Users/abhi3700/F/coding/github_repos/sol_contracts_counter/target/deploy/counter.so...
Program Id: 9b1LijhFmtMDfQAuq9aRVqRiFLeG6Edm5JGiDHZu3AUM

Deploy success
✨  Done in 19.79s.
```

### Mainnet
* `$ yarn deploymain`