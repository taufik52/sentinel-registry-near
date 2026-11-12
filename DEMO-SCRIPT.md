# Demo Script — Sentinel Registry (for X post + video)

Target length: under 90 seconds of video, or a single X post with screenshots.

## X post text (draft)

> 🛡️ Sentinel Registry — token risk attestations, on-chain on @NEARProtocol.
>
> Every agent re-runs the same anti-rug check — and the result evaporates.
> We put the verdict on-chain: score 0–100, PASS/REVIEW/FAIL, evidence link,
> signed by the attester.
>
> One call — get_latest_for_token — before you touch any token.
> Built for the #NEARHackathon (AI Security & Defense + Agent Economy).
>
> Repo: <link>
> Demo: <link>
> Live contract (testnet): sentinel-registry-2026.testnet
> https://testnet.nearblocks.io/accounts/sentinel-registry-2026.testnet

## Video storyboard (~75 s)

1. **0–8 s — The problem.** Screen: a token scanner output / terminal.
   Voiceover: "Every AI agent re-runs the same token safety check — and the
   result disappears."
2. **8–20 s — The contract.** Show `contract/src/lib.rs` + `cargo test`
   output (7 passed). "Sentinel Registry stores the verdict on NEAR:
   score, verdict, evidence, signed by the attester."
3. **20–40 s — Submit.** Frontend: connect testnet wallet, submit an
   attestation for a token (e.g. WBNB on BSC, score 95, PASS). Transaction
   confirms.
4. **40–60 s — Check.** Same frontend, query the token: latest attestation
   appears — PASS 95/100, attester, evidence link. "One view call before you
   touch a token."
5. **60–75 s — Close.** Show README architecture diagram + repo link.
   "Shared memory for token safety. Built for the NEAR Hackathon."

## Screenshots to capture (if no video)

1. `cargo test` — 7 passed.
2. Frontend — check-a-token result (PASS 95/100).
3. Frontend — submit form filled in.
4. NEAR Explorer — the deployed contract + a submit transaction.

## Recording notes

- Record at 1080p, terminal font large enough to read on mobile.
- Keep the wallet/testnet account visible only if it is a throwaway testnet
  account — never show mainnet keys or seed phrases.
- The demo post must be public and tag @NEARProtocol with #NEARHackathon.
