This is the record of myself learning solana program.

To run the stuff in this repo, you have to install this below.
1.  rust
2.  cargo
3.  solana

Each directory has two different sub file,
One is program, and the client is stuff to code to interact with the program you deployed.

Before dive in, start your own solana validator with the following code

`solana-test-validator`
And don't forget to set your solana-cli to localhost
`solana config set --url http://127.0.0.1:8899`

To compile your program, go into the program folder and run the following command
`cargo build-bpf`
will do the work, and deploy you're program after

To compile your code in client side, run
`cargo build && cargo run`
to run you're code.

And you're good to go!

1. Create your own spl token in program
Keypoint:
    In solana, a Mint in spl_token program equals to a ERC-20 standard token.
    To initialize a mint, you have to create a storage to store your mint, that is an account in solana.
    So create a account and set its size as length of mint, to store your mint before initialize mint.
    
    And in the client side, create a account by random, pass it to program as a storage for mint account.
    Because you're program doesn't know where is sysvar::rent, spl_token and system_program, so you have to pass it to the program aswell.
 
