# [FuelEv.ai](https://fuelev.ai) : Car Tokenization Platform

This project is a decentralized application (dApp) for tokenizing cars as NFTs (Non-Fungible Tokens) on the Internet Computer (ICP). It includes a frontend and backend setup with ICP canisters for managing NFT collections and assets.

## Prerequisites

To run this project locally, make sure you have the following installed:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Trunk**: [Install Trunk](https://trunkrs.dev/)
- **DFX (Dfinity SDK)**: [Install DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/)

## Project Structure

### Canister Configuration (`dfx.json`)

```json
{
  "canisters": {
    "fuel-ev-ai": {
      "source": ["dist", "static"],
      "type": "assets"
    }
  }
}
```

# Running Project Locally

## Local Deployment

To deploy the canisters locally using the DFX (Dfinity SDK), run the following script:

```bash
dfx start --background
./scripts/build.sh
./scripts/local_deploy.sh
```
