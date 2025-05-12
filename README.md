# 🧠 GrantChain Core

**GrantChain Core** is the foundational protocol layer for the GrantChain AI-native Layer 2. This repo powers verifiable AI execution, WASM-based smart contract logic, and zkML-integrated consensus systems.

## 📦 Project Structure

- `vm/` - WASM-based smart contract execution engine using Wasmtime
- `sequencer/` - Basic L2 transaction ordering and validation logic
- `zkml/` - Example Circom circuit for zkML-style proof of model inference
- `examples/` - Sample agent + VM execution examples

## 🧪 Features

- Execute smart contracts in a sandboxed WASM runtime
- Define and sort transactions via a custom sequencer
- Include zero-knowledge machine learning proof templates (zkML)
- Modular and extensible Rust architecture

## 🧰 Tech Stack

- Rust (core engine)
- Wasmtime (WASM execution)
- Circom + snarkjs (zkML simulation)
- Solana compatibility layer (future)

## 🚀 Getting Started

```bash
# Build VM
cd vm
cargo build

# Run example
cd ../examples
cargo run
```

## 🧪 Run zkML Proof (Circom)

```bash
cd zkml
circom zk_circuit.circom --r1cs --wasm --sym
snarkjs groth16 setup zk_circuit.r1cs pot.ptau zk_circuit.zkey
snarkjs groth16 prove zk_circuit.zkey input.json proof.json public.json
```

## 📝 License
MIT License © 2025 GrantChain
