# ERC-20 Pallet
### Description

A pallet implementing ERC-20 functionality on top of the substrate-node-template (original docs: https://github.com/substrate-developer-hub/substrate-node-template).
Implements the basic functions such as transfer, approve, transferFrom, increaseAllowance, decreaseAllowance.
The starting point is the init function, where a user can set the total supply, the name of token and its symbol. User, who called init function, gets all obtainable tokens.

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Test

To test the pallet run the following command:

```sh
cargo test --package pallet-erc
```

### Faced problems

- Broken macros
  - Took a lot of time to organize everything just to compile it for the first time.
- Build config broken by Fleet
  - Tried using Fleet instead of Cargo, what led to the broken and irreparable build config. Due to the first successful build, I could not understand the root of the problem and messed with it for several days.
- Usage of BoundedVec in Events
  - For some reason it is impossible to use BoundedVec in Events even with the pallet::without_storage_info macro.
