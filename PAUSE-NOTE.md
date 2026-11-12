# PAUSE NOTE — NEAR Hackathon ("Sentinel Registry on NEAR")

**Status: DI-PAUSE 12 Nov 2026 ~11:30 WIB** atas perintah Cipoz ("Gas WorldIsFlat
sekarang, drop yang lain dulu"). Semua pekerjaan dihentikan. Tidak ada
pendaftaran/submission/pembayaran/login yang dilakukan. Proses `cargo test`
yang masih berjalan sudah dimatikan (SIGTERM); tidak ada proses latar lain
yang berjalan.

## Yang sudah jadi

- **Ide final:** "Sentinel Registry on NEAR" — smart contract NEAR yang
  menyimpan atestasi risiko token (skor pre-check anti-rug 0–100, verdict
  PASS/REVIEW/FAIL, evidence URI) + frontend statis. Track: AI Security &
  Defense + Agent Economy.
- **Contract Rust** (`contract/src/lib.rs`, near-sdk 5.30.1) — lengkap:
  `submit_attestation`, `revoke_attestation` (hanya attester),
  `get_attestation`, `get_attestations_for_token`,
  `get_latest_for_token`, `list_attestations`, `get_stats`; validasi skor
  0–100, key `(chain, address)` case-insensitive. 7 unit test ditulis.
- **Tooling terverifikasi di VM:** Rust 1.99.1 di `~/.cargo/bin` (bukan di
  PATH default), target `wasm32-unknown-unknown` terpasang di toolchain
  **stable** (default aktif adalah nightly — pakai `cargo +stable`),
  Node v24.22.1, near-cli 4.0.13 terpasang di `~/.local` (install biasa
  gagal build native `node-hid`; lolos dengan `--ignore-scripts`).
- **Frontend statis** (`frontend/index.html` + `app.js`, near-api-js 5 via
  CDN): connect wallet testnet, cek token, submit atestasi, daftar terbaru.
  `CONTRACT_ID` masih kosong (menunggu deploy).
- **Dokumen:** README.md, LICENSE (MIT), SUBMISSION.md (draf Inggris),
  DEMO-SCRIPT.md (teks postingan X + storyboard video ~75 dtk).

## Yang BELUM jadi / belum terbukti

1. **`cargo test` BELUM lolos.** Dua masalah ditemukan & diperbaiki di tengah
   jalan, tapi run terakhir (dengan perbaikan) dimatikan paksa saat pause:
   - near-sdk 5 menolak `cargo test` tanpa cargo-near → diperbaiki dengan
     `[dev-dependencies] near-sdk` fitur `unit-testing` (di `Cargo.toml`).
   - `near_sdk::collections` butuh fitur `legacy` di near-sdk 5 → kode
     dimigrasi ke `near_sdk::store::{UnorderedMap, LookupMap}` (API
     `get`→`Option<&V>`, `insert` owned, `iter`→`(&K,&V)`).
   - **Saat dilanjutkan: jalankan `cargo test` dari awal dan catat hasilnya.**
2. **Build WASM belum berhasil** (gagal terakhir karena masalah `collections`
   di atas — sudah diperbaiki di kode, tapi build ulang belum dijalankan).
   Perintah: `cargo +stable build --release --target wasm32-unknown-unknown`.
3. **Deploy testnet BELUM dilakukan** (butuh akun testnet + WASM yang jadi).
4. **Tidak ada repo GitHub, tidak ada postingan X, tidak ada submit Devpost.**

## Daftar file

```
~/workspace/near-hackathon/
├── PAUSE-NOTE.md          (file ini)
├── README.md
├── LICENSE
├── SUBMISSION.md          (draf, ada placeholder repo/demo/alamat payout)
├── DEMO-SCRIPT.md
├── contract/
│   ├── Cargo.toml         (near-sdk 5 + dev-dep unit-testing)
│   ├── Cargo.lock
│   ├── src/lib.rs         (contract + 7 unit test)
│   └── target/            (artefak build, bisa dihapus)
└── frontend/
    ├── index.html
    └── app.js             (CONTRACT_ID = "" — isi setelah deploy)
```

Semua file di atas lengkap dan utuh — tidak ada file setengah-tulis.

---

## RESUME 12 Nov 2026 ~12:22–12:45 WIB — SELESAI (test + build + deploy)

**Hasil:**

1. **`cargo test`: 7 passed, 0 failed ✅** — TAPI run pertama sesi ini
   GAGAL kompilasi (16 error), bukan lolos seperti diasumsikan saat pause.
   Akar masalah: derive `BorshSerialize`/`BorshDeserialize` dari
   `near_sdk::borsh` butuh crate `borsh` sebagai dependensi LANGSUNG
   (`proc_macro_crate::crate_name` panic: "Could not find `borsh` in
   `dependencies`"), sehingga semua pemakaian `UnorderedMap`/`LookupMap`
   gagal E0277/E0599. Perbaikan: tambah `borsh = "1"` di `[dependencies]`
   `contract/Cargo.toml` (resolve ke borsh 1.8.1). Kode kontrak TIDAK
   diubah. Setelah perbaikan, 7/7 lolos di run pertama. Sisa 3 warning
   deprecation `UnorderedMap` → `IterableMap` (non-blocking, dibiarkan).
2. **Build WASM ✅**: `cargo +stable build --release --target
   wasm32-unknown-unknown` → `sentinel_registry.wasm` 331.724 byte,
   SHA-256 `1d03d2d6d773858a201dc4d7024a739512bfa9c6a8949ec399feba656a6802f4`.
3. **Deploy testnet ✅**: akun baru `sentinel-registry-2026.testnet`
   dibuat via `near create-account --useFaucet` (10 testnet NEAR gratis,
   tanpa identitas/KYC/pembayaran Cipoz; key di
   `~/.near-credentials/testnet/sentinel-registry-2026.testnet.json`).
   Deploy + init `new` sukses, tx
   `9n7gAmZ41U35JhxVSgCFtXWwxYEPCdixwx5mZ9zt7tE`.
4. **Verifikasi on-chain ✅**: `get_stats` [0,0] → `submit_attestation`
   WBNB/BSC skor 95 PASS (id 0, tx
   `BsSLztraFxUS6VoXSKtEQ8V5FzdCeEK2DBDwygUvEtPc`) → `get_latest_for_token`
   (query alamat lowercase + chain "BSC" uppercase) kembalikan atestasi —
   lookup case-insensitive terbukti live; `get_stats` [1,1].
5. **Dokumen diperbarui**: `frontend/app.js` CONTRACT_ID diisi akun
   deploy; README.md + SUBMISSION.md (status deploy, link Explorer/tx);
   DEMO-SCRIPT.md (alamat kontrak live di draf postingan X);
   TEST-REPORT.md baru (bukti lengkap test/build/deploy).

**Tersisa (butuh Cipoz):** repo GitHub publik (akun mana), postingan demo
X (siapa posting), alamat NEAR mainnet untuk payout, submit form/API
hackathon, hosting publik frontend (opsional). Tenggat: 15 Nov 2026
06:59 WIB.

## Langkah berikut saat dilanjutkan

1. `cd ~/workspace/near-hackathon/contract && cargo test` → harus 7 passed.
2. `cargo +stable build --release --target wasm32-unknown-unknown` → WASM.
3. Buat akun testnet faucet (tanpa identitas Cipoz, gratis):
   `~/.local/node_modules/.bin/near create-account <nama>.testnet --useFaucet`
   lalu `near deploy` + `near call <akun> new '{}'`.
4. Isi `CONTRACT_ID` di `frontend/app.js`, uji `get_latest_for_token` +
   satu `submit_attestation` di testnet, catat tx hash + link Explorer.
5. Buat repo GitHub publik (butuh keputusan Cipoz: akun mana), publikasikan
   postingan demo di X (tag @NEARProtocol + #NEARHackathon), isi placeholder
   di SUBMISSION.md, lalu submit via form/API Devpost.
6. Tulis TEST-REPORT.md dengan bukti test + deploy.

## Blocker & keputusan yang butuh Cipoz

- **Alamat akun NEAR mainnet untuk payout** — wajib ada sebelum submit;
  hanya Cipoz yang bisa memberikan (jangan dibuatkan).
- **Repo GitHub publik** — akun mana (taufik52 login di VM; taufik420 belum
  login) dan nama repo.
- **Akun testnet untuk deploy** — bisa dibuat gratis via faucet tanpa
  identitas Cipoz; sudah disiapkan jalurnya, tinggal dieksekusi saat lanjut.
- **Postingan demo di X** — butuh akun X Cipoz / keputusan siapa yang posting.
- **Tenggat: 15 Nov 2026 06:59 WIB.** Sisa waktu saat pause: ~3 hari.
