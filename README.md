# Sentinel Registry on NEAR

**On-chain token risk attestations — check a token before you touch it.**

Sentinel Registry is a NEAR smart contract that stores signed risk
attestations for tokens on any chain (BSC, Ethereum, Solana, NEAR). Anyone can
submit an attestation with a risk score (0–100), a verdict (PASS / REVIEW /
FAIL) and a link to the full pre-check evidence. Wallets, scanners and AI
agents read the latest attestation for a token before interacting with it —
turning one-off, off-chain token checks into a shared, tamper-evident public
good.

Built for the **NEAR Hackathon (1–14 Nov 2026)**.
Tracks: **AI Security & Defense** + **Agent Economy**.

## Why this exists

Token pre-checks (anti-rug scoring) are run over and over by every agent and
every user, and the results evaporate. A honeypot flagged once is flagged
again by the next person — or never, if they skip the check. Sentinel
Registry puts the verdict on-chain, keyed by `(chain, token_address)`, so the
work is done once and reused by everyone.

## Architecture

```
┌──────────────┐      submit_attestation       ┌───────────────────────────┐
│  Frontend    │ ───────────────────────────► │  SentinelRegistry (Rust)  │
│  (static     │ ◄─ get_latest_for_token ────  │  near-sdk 5, testnet      │
│   HTML/JS)   │      list_attestations        │  UnorderedMap + LookupMap │
└──────────────┘                              └───────────────────────────┘
```

- **Contract** (`contract/`): Rust, `near-sdk` 5. Stores `Attestation`
  structs; validates score 0–100 and non-empty fields; case-insensitive
  `(chain, address)` keys; the attester alone can revoke their attestation.
- **Frontend** (`frontend/`): dependency-free static page using
  `near-api-js` 5 from CDN. Connect a NEAR testnet wallet, check a token, or
  submit an attestation. No build step — open `index.html` or serve it.

## Contract API

| Method | Type | Description |
|---|---|---|
| `submit_attestation(token_address, chain, score, verdict, evidence_uri)` | change | Submit an attestation, returns id |
| `revoke_attestation(id)` | change | Attester revokes their own attestation |
| `get_attestation(id)` | view | One attestation |
| `get_attestations_for_token(token_address, chain, include_revoked)` | view | All attestations for a token |
| `get_latest_for_token(token_address, chain)` | view | Latest non-revoked attestation |
| `list_attestations(from_index, limit)` | view | Paginated list (max 100/page) |
| `get_stats()` | view | `(total_submitted, stored)` |

## Run the tests

```bash
cd contract
cargo +stable test
```

Expected: **7 passed, 0 failed** (verified 12 Nov 2026 — see `TEST-REPORT.md`).

Build the WASM for deployment:

```bash
cd contract
cargo +stable build --release --target wasm32-unknown-unknown
# output: target/wasm32-unknown-unknown/release/sentinel_registry.wasm
# (verified 12 Nov 2026: 331,724 bytes)
```

## Deploy to testnet

**✅ DEPLOYED 12 Nov 2026** — contract account
`sentinel-registry-2026.testnet` (fresh testnet account created via the
near-cli faucet, funded with 10 testnet NEAR — no owner identity used).
Deploy tx: https://testnet.nearblocks.io/txns/9n7gAmZ41U35JhxVSgCFtXWwxYEPCdixwx5mZ9zt7tE
First attestation (WBNB/BSC, score 95, PASS) tx:
https://testnet.nearblocks.io/txns/BsSLztraFxUS6VoXSKtEQ8V5FzdCeEK2DBDwygUvEtPc
`CONTRACT_ID` in `frontend/app.js` is already set to the deployed account.

## Serve the frontend

```bash
cd frontend
python3 -m http.server 8000
# open http://localhost:8000
```

Set `CONTRACT_ID` in `app.js` first.

## License

MIT — see `LICENSE`.
