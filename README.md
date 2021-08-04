# reflector-duo
Basic CosmWasm (v0.15) Smart Contracts - Demonstrates sending messages from one smart contract to another

Contains two subfolders:
- reflector: Smart Contract that simply reflects messages sent to itself on to another reflectee Smart Contract
- incrementor: Smart Contract that acts as a 'reflectee' and recieves reflected messages for execution from the 'reflector'

## Testing with LocalTerra (main), using terrad (version 0.5.0-rc0-9)
1.Store and Instantiate the reflector contract & note the `contract_address` for both:
  - Store: `terrad tx wasm store ./artifacts/reflector.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block`
  - Instantiate: `terrad tx wasm instantiate <code_id> '{}' --from test1 --chain-id=localterra --fees=100000uluna --gas=auto --broadcast-mode=block`
2. Store and Instantiate the incrementor contract with a starting count of 42:
  - Store: `terrad tx wasm store ./artifacts/incrementor.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block`
  - Instantiate: `terrad tx wasm instantiate <code_id> '{"count":42}' --from test1 --chain-id=localterra --fees=100000uluna --gas=auto --broadcast-mode=block`
3. Set the reflectee address in the reflector contract's state:
  - `terrad tx wasm execute <reflector_contract_addr> '{ "set_reflectee": { "reflectee": "<incrementor_contract_addr>" } }' --from test1 --chain-id=localterra --fees=100000uluna --gas=auto --broadcast-mode=block`
4. Check that the reflector contract's state was updated correctly by querying it
  - `terrad query wasm contract-store <reflector_contract_addr> '{"get_info":{}}'`
5. Using the refector's 'Reflect { msgs: Vec<SubMsg> }', send an 'Increment {}' execute message to the reflectee contract
  - `terrad tx wasm execute <reflector_contract_addr> '{"reflect":{"msgs":[<SubMsg>]}}' --from test1 --chain-id=localterra --fees=100000uluna --gas=auto --broadcast-mode=block`
6. Send an 'GetCount {}' query message to the incrementor contract to check that the reflector forwarding worked
  - `terrad query wasm contract-store <incrementor_contract_addr> '{"get_count":{}}'`
7. Using the reflector contract you can increment the incrementor
  - `terrad tx wasm execute <reflector_contract_addr> '{ "send_increment_to_reflectee": {} }' --from test1 --chain-id=localterra --fees=100000uluna --gas=auto --broadcast-mode=block`
