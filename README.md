# pet-adopt-soroban-contracts

# Requirements:
soroban-cli v0.3.3
```
cargo install --locked --version 0.3.3 soroban-cli
```


# Test
```
cargo test
```

# Build the contract
```
cargo build --target wasm32-unknown-unknown --release
```

# Test de contract
```
cargo test
```
# Run the contract on a sandbox
```
soroban invoke \
    --wasm ./target/wasm32-unknown-unknown/release/pet_adopt_soroban.wasm \
    --id 1 \
    --fn adopt \
    --arg 1
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
echo "Contract deployed with id $PET_ADOPT_ID"

```

Adopt a pet from the terminal

```
soroban invoke \
    --id $PET_ADOPT_ID \
    --secret-key $SECRETKEY \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017' \
    --fn adopt \
    --arg 1
```

Check who is the adopter of pet 1:
```
soroban invoke \
    --id $PET_ADOPT_ID \
    --secret-key $SECRETKEY \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017' \
    --fn adopter \
    --arg 1
```

Check who is the adopter of pet 2:
```
soroban invoke \
    --id $PET_ADOPT_ID \
    --secret-key $SECRETKEY \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017' \
    --fn adopter \
    --arg 1
```

As expected, you'll see that if no one has adopted the pet, the current adopter is the Contract itself (it has been coded this way). You should see something like:

```
["Contract",[220,228,152,154,195,137,188,41,177,27,202,52,159,3,226,232,105,48,20,182,152,122,13,236,11,241,175,157,0,104,99,173]]

```

Erase your secrets from the environment
```
export $(.env.example)
```