This is the record of myself learning solana program.
To run the stuff in this repo, you have to install this below.

1.  [Rust](https://www.rust-lang.org/zh-TW/tools/install)
2.  Cargo
3.  [Solana](https://docs.solana.com/cli/install-solana-cli-tools)

Each directory has two different sub file, One is program, and the client is stuff to code to interact with the program you deployed.

1. program(the on-chain program.)
2. client(interact with the on-chain progarm you deployed.)

Before dive in, start your own solana validator with the following code.

```
# Install Rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

# Reload your PATH environment
$ source $HOME/.cargo/env

# Install Solana-CLI
$ sh -c "$(curl -sSfL https://release.solana.com/v1.7.1/install)"

# Set the Solana Path
$ PATH="/Users/$users/.local/share/solana/install/active_release/bin:$PATH"

# Change the network to localhost:8889
$ solana config set --url http://127.0.0.1:8899

# Start the solana-test-validator
$ solana-test-validator

# Change the dir to program and compile your program 
$ cd program && cargo build-bpf 

# Change the dir to client and run your program And you're good to go!
$ cd .. && cd program
$ cargo build && cargo run

```

## Create your own spl token in program

> Keypoint

In solana, a Mint in spl_token program equals to a ERC-20 standard token.
To initialize a mint, you have to create a storage to store your mint, that is an account in solana.
So create a account and set its size as length of mint, to store your mint before initialize mint.
    
And in the client side, create a account by random, pass it to program as a storage for mint account.
Because you're program doesn't know where is sysvar::rent, spl_token and system_program, so you have to pass it to the program aswell.
 
