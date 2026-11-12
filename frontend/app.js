// Sentinel Registry frontend — NEAR testnet.
// Set CONTRACT_ID to the deployed contract account after `near deploy`.
const CONTRACT_ID = "sentinel-registry-2026.testnet"; // deployed 12 Nov 2026, testnet
const NETWORK = "testnet";

const $ = (id) => document.getElementById(id);
const statusEl = $("status");

let wallet = null;
let account = null;

function setStatus(msg) { statusEl.textContent = msg; }

async function init() {
  if (!CONTRACT_ID) {
    setStatus("Contract not deployed yet. Set CONTRACT_ID in app.js after deploying to testnet.");
    $("check-btn").disabled = true;
    $("submit-btn").disabled = true;
    $("connect-btn").disabled = true;
    return;
  }
  $("contract-id").textContent = CONTRACT_ID;
  const { connect, keyStores, WalletConnection } = nearApi;
  const config = {
    networkId: NETWORK,
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  };
  const near = await connect(config);
  wallet = new WalletConnection(near, "sentinel-registry");
  if (wallet.isSignedIn()) {
    account = wallet.account();
    $("connect-btn").textContent = "Sign out (" + account.accountId + ")";
    setStatus("Signed in as " + account.accountId);
  } else {
    setStatus("Not signed in. Connect your NEAR testnet wallet to submit attestations.");
  }
  loadRecent();
}

async function viewCall(method, args) {
  const res = await account.connection.provider.query({
    request_type: "call_function",
    account_id: CONTRACT_ID,
    method_name: method,
    args_base64: btoa(JSON.stringify(args)),
    finality: "final",
  });
  return JSON.parse(new TextDecoder().decode(new Uint8Array(res.result)));
}

async function checkToken() {
  const chain = $("q-chain").value;
  const token = $("q-token").value.trim();
  if (!token) { $("result").innerHTML = "<p>Enter a token address.</p>"; return; }
  try {
    const att = await viewCall("get_latest_for_token", { token_address: token, chain });
    if (!att) {
      $("result").innerHTML = `<div class="card">No attestation found for <code>${token}</code> on ${chain}. Be careful — no one has attested this token yet.</div>`;
      return;
    }
    $("result").innerHTML = renderAttestation(att);
  } catch (e) {
    $("result").innerHTML = `<p>Error: ${e.message}</p>`;
  }
}

function renderAttestation(a) {
  const cls = a.verdict.toLowerCase();
  const date = new Date(Number(BigInt(a.created_at) / 1000000n)).toLocaleString();
  return `<div class="card">
    <strong class="${cls}">${a.verdict} — score ${a.score}/100</strong><br>
    Token: <code>${a.token_address}</code> (${a.chain})<br>
    Attester: <code>${a.attester}</code><br>
    Submitted: ${date} · id #${a.id}${a.revoked ? " · <strong>REVOKED</strong>" : ""}<br>
    ${a.evidence_uri ? `Evidence: <a href="${a.evidence_uri}" target="_blank" rel="noopener">${a.evidence_uri}</a>` : ""}
  </div>`;
}

async function loadRecent() {
  try {
    const list = await viewCall("list_attestations", { from_index: 0, limit: 10 });
    $("recent").innerHTML = list.length
      ? list.slice().reverse().map(renderAttestation).join("")
      : "<p>No attestations yet.</p>";
  } catch (e) {
    $("recent").innerHTML = `<p>Could not load recent attestations: ${e.message}</p>`;
  }
}

async function submitAttestation() {
  if (!wallet || !wallet.isSignedIn()) { setStatus("Connect your wallet first."); return; }
  const args = {
    token_address: $("s-token").value.trim(),
    chain: $("s-chain").value,
    score: Number($("s-score").value),
    verdict: $("s-verdict").value,
    evidence_uri: $("s-evidence").value.trim() || null,
  };
  if (!args.token_address || args.score < 0 || args.score > 100) {
    setStatus("Check the token address and score (0–100)."); return;
  }
  setStatus("Sending transaction… confirm in your wallet.");
  try {
    await account.functionCall({
      contractId: CONTRACT_ID,
      methodName: "submit_attestation",
      args,
      gas: "30000000000000",
      attachedDeposit: "0",
    });
    setStatus("Attestation submitted! Reloading…");
    loadRecent();
  } catch (e) {
    setStatus("Transaction failed: " + e.message);
  }
}

$("connect-btn").addEventListener("click", () => {
  if (wallet && wallet.isSignedIn()) { wallet.signOut(); location.reload(); }
  else if (wallet) { wallet.requestSignIn({ contractId: CONTRACT_ID }); }
});
$("check-btn").addEventListener("click", checkToken);
$("submit-btn").addEventListener("click", submitAttestation);

init();
