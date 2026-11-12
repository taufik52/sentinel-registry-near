# TEST REPORT — Sentinel Registry on NEAR

**Date:** 12 Nov 2026 (resume run, ~12:22–12:45 WIB)
**Tooling:** Rust stable (via `cargo +stable`; default toolchain is nightly),
near-sdk 5.30.1, near-cli 4.0.13.

## 1. Unit tests — 7 passed, 0 failed ✅

Command: `cd contract && cargo +stable test`

```
running 7 tests
test tests::list_pagination_and_stats ... ok
test tests::empty_token_rejected - should panic ... ok
test tests::score_over_100_rejected - should panic ... ok
test tests::ids_increment_and_latest_wins ... ok
test tests::submit_and_read_back ... ok
test tests::revoke_only_by_attester_and_filters_latest ... ok
test tests::unknown_token_returns_none ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Fix applied (1 line in `contract/Cargo.toml`)

The run that was killed at pause time had **not** actually passed — the
first full run in this session failed to compile with 16 errors. Root
cause: the `BorshSerialize` / `BorshDeserialize` derive macros from
`near_sdk::borsh` resolve the `borsh` crate via `proc_macro_crate`, so
`borsh` must be a **direct** dependency; near-sdk 5 only re-exports it.
The derive panicked with:

> `proc_macro_crate::crate_name` call error: Could not find `borsh` in
> `dependencies` or `dev-dependencies` in `.../contract/Cargo.toml`!

…and every `UnorderedMap`/`LookupMap` use then failed with E0277/E0599
(`Attestation: BorshSerialize` not satisfied).

Fix: added `borsh = "1"` under `[dependencies]` (resolves to borsh
1.8.1, matching near-sdk 5.30.1). No contract code changed. After the
fix, `cargo test` passed 7/7 on the first run.

Remaining warnings (non-blocking): 3× deprecation warning —
`near_sdk::store::UnorderedMap` is deprecated in favour of
`IterableMap` (suboptimal iteration performance). Left as-is: the
contract was never deployed before, so a future migration to
`IterableMap` is a clean swap with no stored data to migrate.

## 2. WASM build ✅

Command: `cargo +stable build --release --target wasm32-unknown-unknown`

- Output: `contract/target/wasm32-unknown-unknown/release/sentinel_registry.wasm`
- Size: 331,724 bytes
- SHA-256: `1d03d2d6d773858a201dc4d7024a739512bfa9c6a8949ec399feba656a6802f4`
- Build finished clean (only the 3 UnorderedMap deprecation warnings).

## 3. Testnet deployment ✅

Fresh testnet account created by tooling — **no owner identity, no KYC,
no payment**:

- Account: `sentinel-registry-2026.testnet`
- Created via `near create-account sentinel-registry-2026.testnet --useFaucet`
- Faucet funded it with **10 testnet NEAR**; key stored locally at
  `~/.near-credentials/testnet/sentinel-registry-2026.testnet.json`
  (testnet-only throwaway key; never a mainnet key).
- Deploy: `near deploy sentinel-registry-2026.testnet <wasm>
  --initFunction new --initArgs '{}'`
- Deploy tx: https://testnet.nearblocks.io/txns/9n7gAmZ41U35JhxVSgCFtXWwxYEPCdixwx5mZ9zt7tE
- Explorer: https://testnet.nearblocks.io/accounts/sentinel-registry-2026.testnet

## 4. Live on-chain verification ✅

- `get_stats` right after deploy: `[ 0, 0 ]`
- `submit_attestation` (WBNB on BSC, score 95, verdict Pass,
  evidence `https://example.com/report/wbnb`) → returned id `0`
  - tx: https://testnet.nearblocks.io/txns/BsSLztraFxUS6VoXSKtEQ8V5FzdCeEK2DBDwygUvEtPc
- `get_latest_for_token` queried with **lowercase address + uppercase
  chain `"BSC"`** returned the attestation (id 0, score 95, Pass,
  attester `sentinel-registry-2026.testnet`, revoked: false) —
  case-insensitive lookup verified live.
- `get_stats` after submit: `[ 1, 1 ]`

## 5. Frontend

`CONTRACT_ID` in `frontend/app.js` set to
`sentinel-registry-2026.testnet`. Not yet served publicly or
end-to-end tested in a browser (needs a wallet sign-in step that only
the owner can do in their browser); the contract-side flows are proven
via near-cli above.

## Still open (needs the owner)

1. Public GitHub repo (which account / repo name).
2. Demo post on X (who posts; needs video/screenshot — storyboard in
   DEMO-SCRIPT.md, now with the live contract address + tx links).
3. NEAR **mainnet** payout address for the submission form.
4. Submit repo + demo links via the hackathon form/API.
5. Optional: public hosting for the frontend (e.g. Netlify) for a
   shareable live-demo URL.

**Deadline: 15 Nov 2026 06:59 WIB.**
