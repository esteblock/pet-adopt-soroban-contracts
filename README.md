# pet-adopt-soroban-contracts

# Test
```
cargo test
```

# Build the contract
```
cargo build --target wasm32-unknown-unknown --release
```
# Run the contract on a sandbox
```
soroban invoke \
    --wasm ../target/wasm32-unknown-unknown/release/pet-adopt-soroban.wasm \
    --id 1 \
    --fn hello \
    --arg friend
```

# Deploy to a local network:
1. First in a terminal run a local network instance
```
docker run --rm -it \
  --platform linux/amd64 \
  -p 8000:8000 \
  --name stellar \
  stellar/quickstart:soroban-dev@sha256:0993d3350148af6ffeab5dc8f0b835236b28dade6dcae77ff8a09317162f768d \
  --standalone \
  --enable-soroban-rpc
```
2. Set your private and public ket in your evironment
```
cp .env.example.env
```
Place your secrets
```
export $(.env)
```
3. Fund your wallet:
```
curl "http://localhost:8000/friendbot?addr=$PUBLICKEY"
```
4. 
```
mkdir -p .soroban
PET_ADOPT_ID="$(
  soroban deploy \
    --wasm target/wasm32-unknown-unknown/release/pet_adopt_soroban.wasm \
    --secret-key $SECRETKEY \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017'
)"
echo "$PET_ADOPT_ID" > .soroban/pet_adopt_id

```



Erase your secrets from the environment
```
export $(.env.example)
```