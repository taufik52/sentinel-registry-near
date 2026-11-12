//! Sentinel Registry — on-chain registry of token risk attestations on NEAR.
//!
//! Anyone can submit a signed attestation for a token (identified by its
//! contract address + chain) with a risk score (0-100), a verdict
//! (PASS / REVIEW / FAIL) and an evidence URI. Attestations are immutable
//! except that the original attester may revoke their own attestation.
//! Consumers (wallets, scanners, agents) read the latest attestation per
//! token before interacting with it.

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{LookupMap, UnorderedMap};
use near_sdk::{env, near, AccountId, PanicOnDefault};

/// Verdict of a token risk pre-check.
#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Verdict {
    Pass,
    Review,
    Fail,
}

/// One risk attestation for a token.
#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Attestation {
    pub id: u64,
    /// Token contract address on its home chain (e.g. BSC 0x... address).
    pub token_address: String,
    /// Home chain identifier, e.g. "bsc", "ethereum", "solana".
    pub chain: String,
    /// Risk score 0..=100 (higher = safer).
    pub score: u8,
    pub verdict: Verdict,
    /// Optional URI pointing at the full pre-check evidence/report.
    pub evidence_uri: Option<String>,
    /// Account that submitted this attestation.
    pub attester: AccountId,
    /// Block timestamp (nanoseconds) when submitted.
    pub created_at: u64,
    /// True once the attester revokes this attestation.
    pub revoked: bool,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct SentinelRegistry {
    attestations: UnorderedMap<u64, Attestation>,
    /// token key ("chain:address" lowercased) -> list of attestation ids, in order.
    by_token: LookupMap<String, Vec<u64>>,
    next_id: u64,
}

fn token_key(chain: &str, token_address: &str) -> String {
    format!("{}:{}", chain.trim().to_lowercase(), token_address.trim().to_lowercase())
}

#[near]
impl SentinelRegistry {
    #[init]
    pub fn new() -> Self {
        Self {
            attestations: UnorderedMap::new(b"a"),
            by_token: LookupMap::new(b"t"),
            next_id: 0,
        }
    }

    /// Submit a new attestation. Returns the assigned attestation id.
    /// Score must be 0..=100, token address and chain must be non-empty.
    pub fn submit_attestation(
        &mut self,
        token_address: String,
        chain: String,
        score: u8,
        verdict: Verdict,
        evidence_uri: Option<String>,
    ) -> u64 {
        assert!(score <= 100, "score must be between 0 and 100");
        let token_address = token_address.trim().to_string();
        let chain = chain.trim().to_string();
        assert!(!token_address.is_empty(), "token_address must not be empty");
        assert!(!chain.is_empty(), "chain must not be empty");
        if let Some(uri) = &evidence_uri {
            assert!(!uri.trim().is_empty(), "evidence_uri must not be empty");
            assert!(uri.len() <= 512, "evidence_uri too long");
        }

        let id = self.next_id;
        self.next_id += 1;

        let attestation = Attestation {
            id,
            token_address: token_address.clone(),
            chain: chain.clone(),
            score,
            verdict,
            evidence_uri,
            attester: env::predecessor_account_id(),
            created_at: env::block_timestamp(),
            revoked: false,
        };
        self.attestations.insert(id, attestation);

        let key = token_key(&chain, &token_address);
        let mut ids = self.by_token.get(&key).cloned().unwrap_or_default();
        ids.push(id);
        self.by_token.insert(key, ids);

        id
    }

    /// Revoke your own attestation. Returns true if revoked.
    pub fn revoke_attestation(&mut self, id: u64) -> bool {
        let mut att = self
            .attestations
            .get(&id)
            .cloned()
            .unwrap_or_else(|| env::panic_str("attestation not found"));
        assert_eq!(
            att.attester,
            env::predecessor_account_id(),
            "only the attester can revoke"
        );
        if att.revoked {
            return false;
        }
        att.revoked = true;
        self.attestations.insert(id, att);
        true
    }

    /// Get one attestation by id.
    pub fn get_attestation(&self, id: u64) -> Option<Attestation> {
        self.attestations.get(&id).cloned()
    }

    /// All attestations for a token (newest last), optionally skipping revoked ones.
    pub fn get_attestations_for_token(
        &self,
        token_address: String,
        chain: String,
        include_revoked: bool,
    ) -> Vec<Attestation> {
        let key = token_key(&chain, &token_address);
        let ids = self.by_token.get(&key).cloned().unwrap_or_default();
        ids.iter()
            .filter_map(|id| self.attestations.get(id).cloned())
            .filter(|a| include_revoked || !a.revoked)
            .collect()
    }

    /// The latest non-revoked attestation for a token, if any.
    pub fn get_latest_for_token(
        &self,
        token_address: String,
        chain: String,
    ) -> Option<Attestation> {
        self.get_attestations_for_token(token_address, chain, false)
            .into_iter()
            .last()
    }

    /// Paginated list of all attestations (oldest first).
    pub fn list_attestations(&self, from_index: u64, limit: u64) -> Vec<Attestation> {
        let limit = limit.min(100);
        self.attestations
            .iter()
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|(_, att)| att.clone())
            .collect()
    }

    /// Registry statistics.
    pub fn get_stats(&self) -> (u64, u64) {
        (self.next_id, self.attestations.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn ctx(predecessor: AccountId) -> VMContextBuilder {
        let mut b = VMContextBuilder::new();
        b.predecessor_account_id(predecessor);
        b.block_timestamp(1_700_000_000_000_000_000);
        b
    }

    fn setup() -> SentinelRegistry {
        testing_env!(ctx(accounts(0)).build());
        SentinelRegistry::new()
    }

    #[test]
    fn submit_and_read_back() {
        let mut c = setup();
        testing_env!(ctx(accounts(0)).build());
        let id = c.submit_attestation(
            "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bC095c".to_string(),
            "bsc".to_string(),
            95,
            Verdict::Pass,
            Some("https://example.com/report/wbnb".to_string()),
        );
        assert_eq!(id, 0);
        let att = c.get_attestation(0).expect("attestation exists");
        assert_eq!(att.score, 95);
        assert_eq!(att.verdict, Verdict::Pass);
        assert_eq!(att.attester, accounts(0));
        assert!(!att.revoked);
        assert_eq!(att.created_at, 1_700_000_000_000_000_000);
    }

    #[test]
    fn ids_increment_and_latest_wins() {
        let mut c = setup();
        testing_env!(ctx(accounts(0)).build());
        c.submit_attestation("0xAAA".into(), "bsc".into(), 40, Verdict::Review, None);
        testing_env!(ctx(accounts(1)).build());
        let id2 = c.submit_attestation("0xAAA".into(), "BSC".into(), 20, Verdict::Fail, None);
        assert_eq!(id2, 1);
        // chain/address lookup is case-insensitive
        let latest = c
            .get_latest_for_token("0xaaa".into(), "bsc".into())
            .expect("latest exists");
        assert_eq!(latest.id, 1);
        assert_eq!(latest.score, 20);
        let all = c.get_attestations_for_token("0xAAA".into(), "bsc".into(), false);
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn revoke_only_by_attester_and_filters_latest() {
        let mut c = setup();
        testing_env!(ctx(accounts(0)).build());
        c.submit_attestation("0xBBB".into(), "ethereum".into(), 80, Verdict::Pass, None);
        // someone else cannot revoke
        testing_env!(ctx(accounts(1)).build());
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            c.revoke_attestation(0)
        }));
        assert!(r.is_err(), "non-attester revoke must panic");
        // attester revokes
        testing_env!(ctx(accounts(0)).build());
        assert!(c.revoke_attestation(0));
        assert!(!c.revoke_attestation(0)); // second revoke is a no-op
        let att = c.get_attestation(0).unwrap();
        assert!(att.revoked);
        assert!(c.get_latest_for_token("0xBBB".into(), "ethereum".into()).is_none());
        assert_eq!(
            c.get_attestations_for_token("0xBBB".into(), "ethereum".into(), true)
                .len(),
            1
        );
    }

    #[test]
    #[should_panic(expected = "score must be between 0 and 100")]
    fn score_over_100_rejected() {
        let mut c = setup();
        testing_env!(ctx(accounts(0)).build());
        c.submit_attestation("0xCCC".into(), "bsc".into(), 101, Verdict::Pass, None);
    }

    #[test]
    #[should_panic(expected = "token_address must not be empty")]
    fn empty_token_rejected() {
        let mut c = setup();
        testing_env!(ctx(accounts(0)).build());
        c.submit_attestation("   ".into(), "bsc".into(), 50, Verdict::Review, None);
    }

    #[test]
    fn list_pagination_and_stats() {
        let mut c = setup();
        testing_env!(ctx(accounts(0)).build());
        for i in 0..5u8 {
            c.submit_attestation(
                format!("0x{:03}", i),
                "bsc".into(),
                50 + i,
                Verdict::Review,
                None,
            );
        }
        let (total, stored) = c.get_stats();
        assert_eq!((total, stored), (5, 5));
        let page = c.list_attestations(2, 2);
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].id, 2);
        assert_eq!(page[1].id, 3);
        let tail = c.list_attestations(4, 100);
        assert_eq!(tail.len(), 1);
        assert!(c.list_attestations(99, 10).is_empty());
    }

    #[test]
    fn unknown_token_returns_none() {
        let c = setup();
        assert!(c.get_latest_for_token("0xNOPE".into(), "bsc".into()).is_none());
        assert!(c.get_attestation(42).is_none());
    }
}
