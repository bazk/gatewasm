# GateWASM

## 1. Start API Gateway Server
```
cd core
cargo run
```

> Runs at port 8700 and *8900*

## 2. Start Web UI

```
cd web-ui
yarn start
```

> Runs at port *3000*

## 3. Build code from an example

```
cd core/examples/yell-api-c
make
```

OR

```
cd core/examples/yell-api-rust
cargo build --target wasm32-unknown-unknown --release
```

## 3. Create a route

At http://localhost:3000


## 4. Call the API

curl -XPOST -d'{"text": "terminei só falta testar"}' localhost:8700/yell