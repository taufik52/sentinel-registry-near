# NEAR Hackathon — Submission Draft

> STATUS 12 Nov 2026 ~14:00 WIB — Repo publik JADI, frontend live JADI,
> payout `cipo420.tg` terisi. TERTAHAN: postingan demo X belum terbit
> (login X desktop kena limit; draf siap di X-POST-DRAFT.md) — link
> postingan X WAJIB untuk form submission di nearhackathon.com.

## Title

**Sentinel Registry — On-Chain Token Risk Attestations on NEAR**

## Tracks

- AI Security & Defense (primary)
- Agent Economy

## Short description

Sentinel Registry is a NEAR smart contract that stores signed token risk
attestations (score 0–100, verdict PASS / REVIEW / FAIL, evidence link) keyed
by token address and chain. It turns one-off, off-chain anti-rug pre-checks
into a shared, tamper-evident public registry that wallets, scanners and AI
agents can query before touching a token.

## Long description

Every day, AI agents and users run the same token safety checks — is this a
honeypot, is the deployer a known rugger, is liquidity locked? — and the
results evaporate. The next agent repeats the work, or skips it and gets
rugged.

Sentinel Registry puts the verdict on-chain. Anyone can submit an attestation
for any token on any chain (BSC, Ethereum, Solana, NEAR) with a risk score, a
verdict and a link to the full evidence report. The attestation is signed by
the submitter's NEAR account, timestamped, and revocable only by its author.
Consumers call one view method — `get_latest_for_token` — before interacting
with a token.

The design is deliberately minimal and chain-agnostic: the contract stores
attestations, not opinions about how to compute them, so any scoring model
(rule-based, ML, or human audit) can publish to the same registry. It is the
missing shared memory layer for the agent economy's token safety tooling.

## How to run

**Tests (verified 12 Nov 2026, 7 passed, 0 failed — see TEST-REPORT.md):**

```bash
cd contract
cargo +stable test   # 7 passed, 0 failed
```

**Build the contract (verified 12 Nov 2026, WASM 331,724 bytes):**

```bash
cd contract
cargo +stable build --release --target wasm32-unknown-unknown
```

**Live on testnet (deployed 12 Nov 2026):**

- Contract: `sentinel-registry-2026.testnet`
- Deploy tx: https://testnet.nearblocks.io/txns/9n7gAmZ41U35JhxVSgCFtXWwxYEPCdixwx5mZ9zt7tE
- First attestation (WBNB/BSC, score 95, PASS):
  https://testnet.nearblocks.io/txns/BsSLztraFxUS6VoXSKtEQ8V5FzdCeEK2DBDwygUvEtPc
- Explorer account: https://testnet.nearblocks.io/accounts/sentinel-registry-2026.testnet

**Frontend:** static HTML/JS, no build step. `CONTRACT_ID` in
`frontend/app.js` is already set to the deployed testnet contract
(`sentinel-registry-2026.testnet`); serve with:

```bash
cd frontend && python3 -m http.server 8000
```

**Deploy (testnet):** done 12 Nov 2026 — `sentinel-registry-2026.testnet`
(see README.md / TEST-REPORT.md).

## Architecture

- **Smart contract** (`contract/src/lib.rs`, Rust, near-sdk 5): `Attestation`
  struct; `UnorderedMap<u64, Attestation>` for storage plus a
  `LookupMap<String, Vec<u64>>` index keyed by lowercased
  `chain:token_address` for case-insensitive lookup. Input validation:
  score 0–100, non-empty token/chain, evidence URI ≤ 512 chars. Revocation
  restricted to the original attester.
- **Frontend** (`frontend/`): static page, near-api-js 5 via CDN, wallet
  connect, check-a-token and submit-attestation flows.
- **No backend, no secrets, no off-chain database** — the contract is the
  whole backend.

## What is new

All code was written fresh for this hackathon (1–14 Nov 2026). No code was
reused from prior projects; the scoring concept builds on the author's
existing token pre-check work, but the contract, tests and frontend are new.

## Links

- Repository: https://github.com/taufik52/sentinel-registry-near (public)
- Demo post on X: _(public post tagging @NEARProtocol and #NEARHackathon — to be published)_
- Live demo: https://sentinel-registry-near.netlify.app (frontend static, points at `sentinel-registry-2026.testnet`; deployed 12 Nov 2026)
- Contract (testnet): `sentinel-registry-2026.testnet` — https://testnet.nearblocks.io/accounts/sentinel-registry-2026.testnet
- NEAR mainnet payout address: `cipo420.tg`

## Still needed before submitting

1. ~~Public GitHub repo~~ — DONE 12 Nov 2026: https://github.com/taufik52/sentinel-registry-near
2. **Demo post on X** — DRAFT READY (X-POST-DRAFT.md, 256 chars + 2 screenshots);
   needs posting from Cipoz's phone (@DongCape, session still logged in) or a
   browser attempt after the desktop login limit clears. Link postingan WAJIB
   untuk form.
3. ~~NEAR mainnet payout address~~ — DONE: `cipo420.tg` (perintah Cipoz 13:51 WIB 12 Nov).
4. Submit repo link + demo post link via the form at https://nearhackathon.com
   (Final Submission section; fields: project name, description 20–2.000 chars,
   team lead, email, team size = Solo builder, tracks = AI Security & Defense +
   Agent Economy, payout address, repo link, X post link). No wallet connection
   needed. One submission per project, cannot be edited after sending (a
   "Request changes" page exists for corrections).

(Done 12 Nov 2026: unit tests 7/7, WASM build, testnet deployment +
live attestation — see TEST-REPORT.md.)
