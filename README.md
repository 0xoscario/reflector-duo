# rusty-reflector
Basic CosmWasm (v0.15) Smart Contracts - Demonstrates sending messages from one smart contract to another

Contains two subfolders:
- reflector: Smart Contract that simply reflects messages sent to itself on to another reflectee Smart Contract
- incrementor: Smart Contract that acts as a 'reflectee' and recieves reflected messages for execution from the 'reflector'

## Testing with LocalTerra (main), using terrad (version 0.5.0-rc0-9)
1.Store and Instantiate the reflector and reflectee contracts: 
  - Store: `terrad tx wasm store ./artifacts/<file_name>.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block`
  - Instantiate: `terrad tx wasm instantiate <code_id> '{}' --from test1 --chain-id=localterra --fees=100000uluna --gas=auto --broadcast-mode=block`
2. Set the reflectee address in the reflector contract's state: 
  - ``
3. Send an 'Increment {}' execute message to the reflectee contract via the refector's 'Reflect { msgs: Vec<SubMsg>}'
  - ``
