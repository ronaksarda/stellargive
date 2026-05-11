# stellarGive (Soroban Contract)

`stellarGive` is a Soroban smart contract for transparent token-based relief campaigns on Stellar Testnet.

## Architecture Overview

- **Campaign lifecycle**: `Active -> Funded -> Claimed | Expired`
- **Persistent storage**:
  - `symbol_short!("NEXT")` for campaign ID sequencing
  - `(symbol_short!("CMP"), campaign_id)` for campaign records
- **Core methods**:
  - `create_campaign`: creates campaign metadata and validation gates
  - `donate`: transfers tokens from donor to contract and updates raised amount
  - `claim_funds`: allows creator/beneficiary to send all raised funds to beneficiary
  - `get_campaign`: view full campaign state with status derived from ledger time

## Security Notes

- `#![no_std]` and Soroban `#[contract]`/`#[contractimpl]` patterns.
- All state-changing methods enforce caller auth with `require_auth()`.
- Input validation for amount/deadline/title/token interface checks.
- Explicit `ContractError` enum for predictable error handling.
- Reentrancy guard via temporary lock storage key (`symbol_short!("LOCK")`).
- Overflow-safe arithmetic with `checked_add` and `overflow-checks = true`.
- Structured events emitted for:
  - `("campaign", "created")`
  - `("donation", "received")`
  - `("funds", "claimed")`

## Build & Test

```bash
cd contracts/stellar-give
make test
make wasm
```

## Stellar Testnet Setup

1. Install Stellar CLI and configure testnet:
```bash
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

2. Create an identity (example: `alice`) and fund it:
```bash
stellar keys generate alice
curl "https://friendbot.stellar.org/?addr=$(stellar keys address alice)"
```

3. Build and deploy:
```bash
cd contracts/stellar-give
make deploy SOURCE=alice
```

## Invoke Examples

```bash
make invoke-create \
  SOURCE=alice \
  CONTRACT_ID=<CONTRACT_ID> \
  CREATOR=<CREATOR_ADDRESS> \
  BENEFICIARY=<BENEFICIARY_ADDRESS> \
  TOKEN=<TOKEN_CONTRACT_ADDRESS> \
  TARGET=5000000 \
  DEADLINE=2000000000
```

```bash
make invoke-donate \
  SOURCE=<DONOR_IDENTITY> \
  CONTRACT_ID=<CONTRACT_ID> \
  DONOR=<DONOR_ADDRESS> \
  CAMPAIGN_ID=1 \
  AMOUNT=1000000
```

```bash
make invoke-claim \
  SOURCE=<CREATOR_OR_BENEFICIARY_IDENTITY> \
  CONTRACT_ID=<CONTRACT_ID> \
  CAMPAIGN_ID=1
```
