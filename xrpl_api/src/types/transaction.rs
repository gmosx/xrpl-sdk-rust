mod common;
mod variants;

use serde::{Deserialize, Serialize};

pub use common::*;

pub use variants::account_delete::*;
pub use variants::account_set::*;
pub use variants::offer_cancel::*;
pub use variants::offer_create::*;
pub use variants::payment::*;
pub use variants::trust_set::*;

/// Ledger transaction. See <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "TransactionType")]
pub enum Transaction {
    AccountDelete(AccountDeleteTransaction),
    AccountSet(AccountSetTransaction),
    // TODO add model for remaining transactions
    CheckCancel(TransactionCommon),
    CheckCash(TransactionCommon),
    CheckCreate(TransactionCommon),
    DepositPreauth(TransactionCommon),
    EscrowCancel(TransactionCommon),
    EscrowCreate(TransactionCommon),
    EscrowFinish(TransactionCommon),
    NFTokenAcceptOffer(TransactionCommon),
    NFTokenBurn(TransactionCommon),
    NFTokenCancelOffer(TransactionCommon),
    NFTokenCreateOffer(TransactionCommon),
    NFTokenMint(TransactionCommon),
    OfferCancel(OfferCancelTransaction),
    OfferCreate(OfferCreateTransaction),
    Payment(PaymentTransaction),
    PaymentChannelClaim(TransactionCommon),
    PaymentChannelCreate(TransactionCommon),
    PaymentChannelFund(TransactionCommon),
    SetRegularKey(TransactionCommon),
    SignerListSet(TransactionCommon),
    TicketCreate(TransactionCommon),
    TrustSet(TrustSetTransaction),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    AMMBid(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    AMMCreate(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    AMMDelete(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    AMMDeposit(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    AMMVote(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    AMMWithdraw(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    Clawback(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    DIDDelete(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    DIDSet(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    OracleDelete(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    OracleSet(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainAccountCreateCommit(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainAddAccountCreateAttestation(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainAddClaimAttestation(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainClaim(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainCommit(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainCreateBridge(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainCreateClaimID(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    XChainModifyBridge(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    EnableAmendment(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    SetFee(TransactionCommon),
    // BASELINE: variant-specific fields are not modelled and are silently dropped
    UNLModify(TransactionCommon),
    #[serde(other)]
    Unknown,
}

impl Transaction {
    pub fn common(&self) -> &TransactionCommon {
        match self {
            Transaction::AccountDelete(t) => &t.common,
            Transaction::AccountSet(t) => &t.common,
            Transaction::OfferCancel(t) => &t.common,
            Transaction::OfferCreate(t) => &t.common,
            Transaction::Payment(t) => &t.common,
            Transaction::TrustSet(t) => &t.common,
            Transaction::CheckCancel(t) => t,
            Transaction::CheckCash(t) => t,
            Transaction::CheckCreate(t) => t,
            Transaction::DepositPreauth(t) => t,
            Transaction::EscrowCancel(t) => t,
            Transaction::EscrowCreate(t) => t,
            Transaction::EscrowFinish(t) => t,
            Transaction::NFTokenAcceptOffer(t) => t,
            Transaction::NFTokenBurn(t) => t,
            Transaction::NFTokenCancelOffer(t) => t,
            Transaction::NFTokenCreateOffer(t) => t,
            Transaction::NFTokenMint(t) => t,
            Transaction::PaymentChannelClaim(t) => t,
            Transaction::PaymentChannelCreate(t) => t,
            Transaction::PaymentChannelFund(t) => t,
            Transaction::SetRegularKey(t) => t,
            Transaction::SignerListSet(t) => t,
            Transaction::TicketCreate(t) => t,
            Transaction::AMMBid(t) => t,
            Transaction::AMMCreate(t) => t,
            Transaction::AMMDelete(t) => t,
            Transaction::AMMDeposit(t) => t,
            Transaction::AMMVote(t) => t,
            Transaction::AMMWithdraw(t) => t,
            Transaction::Clawback(t) => t,
            Transaction::DIDDelete(t) => t,
            Transaction::DIDSet(t) => t,
            Transaction::OracleDelete(t) => t,
            Transaction::OracleSet(t) => t,
            Transaction::XChainAccountCreateCommit(t) => t,
            Transaction::XChainAddAccountCreateAttestation(t) => t,
            Transaction::XChainAddClaimAttestation(t) => t,
            Transaction::XChainClaim(t) => t,
            Transaction::XChainCommit(t) => t,
            Transaction::XChainCreateBridge(t) => t,
            Transaction::XChainCreateClaimID(t) => t,
            Transaction::XChainModifyBridge(t) => t,
            Transaction::EnableAmendment(t) => t,
            Transaction::SetFee(t) => t,
            Transaction::UNLModify(t) => t,
            Transaction::Unknown => unreachable!("common() is not available on Transaction::Unknown — this variant has no associated TransactionCommon"),
        }
    }
}

impl Transaction {
    pub fn common_mut(&mut self) -> &mut TransactionCommon {
        match self {
            Transaction::AccountDelete(t) => &mut t.common,
            Transaction::AccountSet(t) => &mut t.common,
            Transaction::OfferCancel(t) => &mut t.common,
            Transaction::OfferCreate(t) => &mut t.common,
            Transaction::Payment(t) => &mut t.common,
            Transaction::TrustSet(t) => &mut t.common,
            Transaction::CheckCancel(t) => t,
            Transaction::CheckCash(t) => t,
            Transaction::CheckCreate(t) => t,
            Transaction::DepositPreauth(t) => t,
            Transaction::EscrowCancel(t) => t,
            Transaction::EscrowCreate(t) => t,
            Transaction::EscrowFinish(t) => t,
            Transaction::NFTokenAcceptOffer(t) => t,
            Transaction::NFTokenBurn(t) => t,
            Transaction::NFTokenCancelOffer(t) => t,
            Transaction::NFTokenCreateOffer(t) => t,
            Transaction::NFTokenMint(t) => t,
            Transaction::PaymentChannelClaim(t) => t,
            Transaction::PaymentChannelCreate(t) => t,
            Transaction::PaymentChannelFund(t) => t,
            Transaction::SetRegularKey(t) => t,
            Transaction::SignerListSet(t) => t,
            Transaction::TicketCreate(t) => t,
            Transaction::AMMBid(t) => t,
            Transaction::AMMCreate(t) => t,
            Transaction::AMMDelete(t) => t,
            Transaction::AMMDeposit(t) => t,
            Transaction::AMMVote(t) => t,
            Transaction::AMMWithdraw(t) => t,
            Transaction::Clawback(t) => t,
            Transaction::DIDDelete(t) => t,
            Transaction::DIDSet(t) => t,
            Transaction::OracleDelete(t) => t,
            Transaction::OracleSet(t) => t,
            Transaction::XChainAccountCreateCommit(t) => t,
            Transaction::XChainAddAccountCreateAttestation(t) => t,
            Transaction::XChainAddClaimAttestation(t) => t,
            Transaction::XChainClaim(t) => t,
            Transaction::XChainCommit(t) => t,
            Transaction::XChainCreateBridge(t) => t,
            Transaction::XChainCreateClaimID(t) => t,
            Transaction::XChainModifyBridge(t) => t,
            Transaction::EnableAmendment(t) => t,
            Transaction::SetFee(t) => t,
            Transaction::UNLModify(t) => t,
            Transaction::Unknown => unreachable!("common() is not available on Transaction::Unknown — this variant has no associated TransactionCommon"),
        }
    }
}

// BASELINE: tests for missing TransactionType variants — see issue #41
#[cfg(test)]
mod tests {
    use super::Transaction;

    // --- AMM family ---

    #[test]
    fn test_deserialize_amm_bid() {
        let json = r#"{"TransactionType":"AMMBid","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_create() {
        let json = r#"{"TransactionType":"AMMCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_delete() {
        let json = r#"{"TransactionType":"AMMDelete","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_deposit() {
        let json = r#"{"TransactionType":"AMMDeposit","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_vote() {
        let json = r#"{"TransactionType":"AMMVote","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_withdraw() {
        let json = r#"{"TransactionType":"AMMWithdraw","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Clawback ---

    #[test]
    fn test_deserialize_clawback() {
        let json = r#"{"TransactionType":"Clawback","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- DID family ---

    #[test]
    fn test_deserialize_did_delete() {
        let json = r#"{"TransactionType":"DIDDelete","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_did_set() {
        let json = r#"{"TransactionType":"DIDSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Oracle family ---

    #[test]
    fn test_deserialize_oracle_delete() {
        let json = r#"{"TransactionType":"OracleDelete","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_oracle_set() {
        let json = r#"{"TransactionType":"OracleSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- XChain family ---

    #[test]
    fn test_deserialize_x_chain_account_create_commit() {
        let json = r#"{"TransactionType":"XChainAccountCreateCommit","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_add_account_create_attestation() {
        let json = r#"{"TransactionType":"XChainAddAccountCreateAttestation","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_add_claim_attestation() {
        let json = r#"{"TransactionType":"XChainAddClaimAttestation","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_claim() {
        let json = r#"{"TransactionType":"XChainClaim","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_commit() {
        let json = r#"{"TransactionType":"XChainCommit","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_create_bridge() {
        let json = r#"{"TransactionType":"XChainCreateBridge","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_create_claim_id() {
        let json = r#"{"TransactionType":"XChainCreateClaimID","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_modify_bridge() {
        let json = r#"{"TransactionType":"XChainModifyBridge","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Pseudo-transactions ---

    #[test]
    fn test_deserialize_enable_amendment() {
        let json = r#"{"TransactionType":"EnableAmendment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_set_fee() {
        let json = r#"{"TransactionType":"SetFee","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_unl_modify() {
        let json = r#"{"TransactionType":"UNLModify","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Unknown catchall (currently FAILS — no catchall variant exists) ---

    #[test]
    fn test_deserialize_unknown_variant_does_not_panic() {
        // Currently FAILS — no catchall variant exists
        let json = r#"{"TransactionType":"SomeFutureVariant","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        let tx: Transaction = serde_json::from_str(json).unwrap();
        // After fix: should deserialize to Transaction::Unknown
        // common() on Unknown is not intended — do not call it here
        let _ = tx; // silence unused warning
    }

    // --- C2: real-world fixture test — variant-specific fields are silently dropped ---
    //
    // AMMCreate carries variant-specific fields (Amount, Amount2, TradingFee) that are
    // NOT modelled in TransactionCommon (BASELINE comment on the variant).  This test
    // uses a realistic ledger payload and asserts:
    //   1. Deserialization succeeds (no Err, no panic).
    //   2. TransactionCommon fields are correct.
    //   3. The variant-specific fields are not accessible — they are silently dropped —
    //      which is the documented lossy contract for BASELINE variants.
    //
    // Payload is representative of the AMMCreate transaction format described at
    // https://xrpl.org/ammcreate.html (XRPL docs, audited 2026-04-05).
    #[test]
    fn test_amm_create_real_payload_common_fields_correct_variant_fields_dropped() {
        let json = r#"{
            "TransactionType": "AMMCreate",
            "Account": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
            "Fee": "10",
            "Sequence": 1,
            "Amount": {
                "currency": "USD",
                "issuer": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
                "value": "100"
            },
            "Amount2": "100000000",
            "TradingFee": 500
        }"#;

        let tx: Transaction = serde_json::from_str(json).expect("AMMCreate should deserialise");

        // Must be the AMMCreate variant — not Unknown
        assert!(
            matches!(tx, Transaction::AMMCreate(_)),
            "Expected Transaction::AMMCreate, got {tx:?}"
        );

        // TransactionCommon fields are intact
        let common = tx.common();
        assert_eq!(common.account, "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
        assert_eq!(common.fee, "10");
        assert_eq!(common.sequence, 1);

        // Variant-specific fields (Amount, Amount2, TradingFee) are not modelled.
        // They are silently dropped — this is the BASELINE lossy contract.
        // Re-serialise and confirm they are absent from the output.
        let reserialized = serde_json::to_string(&tx).expect("should serialise");
        assert!(
            !reserialized.contains("Amount2"),
            "Amount2 must be silently dropped: {reserialized}"
        );
        assert!(
            !reserialized.contains("TradingFee"),
            "TradingFee must be silently dropped: {reserialized}"
        );
    }

    // OracleSet — PriceDataSeries is an array of nested PriceData objects.
    // Variant-specific fields: OracleDocumentID, Provider, URI, LastUpdateTime,
    // AssetClass, PriceDataSeries (array).
    // Payload from https://xrpl.org/docs/references/protocol/transactions/types/oracleset
    // (XRPL docs, audited 2026-04-05).
    #[test]
    fn test_oracle_set_real_payload_nested_array_silently_dropped() {
        let json = r#"{
            "TransactionType": "OracleSet",
            "Account": "roosteri9aGNFRXZrJNYQKVBfxHiE5abg",
            "AssetClass": "63757272656E6379",
            "Fee": "12",
            "LastUpdateTime": 1760397040,
            "OracleDocumentID": 3,
            "PriceDataSeries": [
                {
                    "PriceData": {
                        "AssetPrice": "267e",
                        "BaseAsset": "XRP",
                        "QuoteAsset": "USD",
                        "Scale": 3
                    }
                },
                {
                    "PriceData": {
                        "AssetPrice": "a34",
                        "BaseAsset": "XRP",
                        "QuoteAsset": "EUR",
                        "Scale": 3
                    }
                }
            ],
            "Provider": "7468726565787270",
            "Sequence": 95076881,
            "URI": "68747470733A2F2F6578616D706C652E636F6D"
        }"#;

        let tx: Transaction = serde_json::from_str(json).expect("OracleSet should deserialise");

        assert!(
            matches!(tx, Transaction::OracleSet(_)),
            "Expected Transaction::OracleSet, got {tx:?}"
        );

        let common = tx.common();
        assert_eq!(common.account, "roosteri9aGNFRXZrJNYQKVBfxHiE5abg");
        assert_eq!(common.fee, "12");
        assert_eq!(common.sequence, 95076881);

        // Variant-specific fields including the nested PriceDataSeries array
        // are silently dropped — BASELINE lossy contract.
        let reserialized = serde_json::to_string(&tx).expect("should serialise");
        assert!(
            !reserialized.contains("PriceDataSeries"),
            "PriceDataSeries must be silently dropped: {reserialized}"
        );
        assert!(
            !reserialized.contains("OracleDocumentID"),
            "OracleDocumentID must be silently dropped: {reserialized}"
        );
        assert!(
            !reserialized.contains("LastUpdateTime"),
            "LastUpdateTime must be silently dropped: {reserialized}"
        );
    }

    // XChainCreateBridge — XChainBridge is a nested object with 4 sub-fields.
    // Variant-specific fields: XChainBridge (nested), SignatureReward,
    // MinAccountCreateAmount.
    // Payload from https://xrpl.org/docs/references/protocol/transactions/types/xchaincreatebridge
    // (XRPL docs, audited 2026-04-05).
    #[test]
    fn test_x_chain_create_bridge_real_payload_nested_object_silently_dropped() {
        let json = r#"{
            "TransactionType": "XChainCreateBridge",
            "Account": "rhWQzvdmhf5vFS35vtKUSUwNZHGT53qQsg",
            "Fee": "12",
            "Sequence": 1,
            "XChainBridge": {
                "LockingChainDoor": "rhWQzvdmhf5vFS35vtKUSUwNZHGT53qQsg",
                "LockingChainIssue": { "currency": "XRP" },
                "IssuingChainDoor": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
                "IssuingChainIssue": { "currency": "XRP" }
            },
            "SignatureReward": 200,
            "MinAccountCreateAmount": 1000000
        }"#;

        let tx: Transaction =
            serde_json::from_str(json).expect("XChainCreateBridge should deserialise");

        assert!(
            matches!(tx, Transaction::XChainCreateBridge(_)),
            "Expected Transaction::XChainCreateBridge, got {tx:?}"
        );

        let common = tx.common();
        assert_eq!(common.account, "rhWQzvdmhf5vFS35vtKUSUwNZHGT53qQsg");
        assert_eq!(common.fee, "12");
        assert_eq!(common.sequence, 1);

        // Variant-specific fields including the nested XChainBridge object
        // are silently dropped — BASELINE lossy contract.
        let reserialized = serde_json::to_string(&tx).expect("should serialise");
        assert!(
            !reserialized.contains("XChainBridge"),
            "XChainBridge must be silently dropped: {reserialized}"
        );
        assert!(
            !reserialized.contains("SignatureReward"),
            "SignatureReward must be silently dropped: {reserialized}"
        );
        assert!(
            !reserialized.contains("MinAccountCreateAmount"),
            "MinAccountCreateAmount must be silently dropped: {reserialized}"
        );
    }

    // AMMDeposit — double-asset deposit mode (tfTwoAsset).
    // Variant-specific fields: Asset, Asset2, Amount (token), Amount2 (XRP), Flags.
    // Payload from https://xrpl.org/docs/references/protocol/transactions/types/ammdeposit
    // (XRPL docs, audited 2026-04-05).
    #[test]
    fn test_amm_deposit_real_payload_multi_field_silently_dropped() {
        let json = r#"{
            "TransactionType": "AMMDeposit",
            "Account": "rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm",
            "Fee": "10",
            "Sequence": 7,
            "Flags": 1048576,
            "Amount": {
                "currency": "TST",
                "issuer": "rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd",
                "value": "2.5"
            },
            "Amount2": "30000000",
            "Asset": {
                "currency": "TST",
                "issuer": "rP9jPyP5kyvFRb6ZiRghAGw5u8SGAmU4bd"
            },
            "Asset2": {
                "currency": "XRP"
            }
        }"#;

        let tx: Transaction = serde_json::from_str(json).expect("AMMDeposit should deserialise");

        assert!(
            matches!(tx, Transaction::AMMDeposit(_)),
            "Expected Transaction::AMMDeposit, got {tx:?}"
        );

        let common = tx.common();
        assert_eq!(common.account, "rJVUeRqDFNs2xqA7ncVE6ZoAhPUoaJJSQm");
        assert_eq!(common.fee, "10");
        assert_eq!(common.sequence, 7);

        // Variant-specific fields (Asset, Asset2, Amount, Amount2, Flags as AMM flag)
        // are silently dropped — BASELINE lossy contract.
        let reserialized = serde_json::to_string(&tx).expect("should serialise");
        assert!(
            !reserialized.contains("Asset2"),
            "Asset2 must be silently dropped: {reserialized}"
        );
        assert!(
            !reserialized.contains("Amount2"),
            "Amount2 must be silently dropped: {reserialized}"
        );
    }

    // --- Regression: existing variants must survive the fix ---

    #[test]
    fn test_deserialize_existing_variants_still_work() {
        // These must continue to pass after the fix
        let variants = vec![
            r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
            r#"{"TransactionType":"EscrowCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
            r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
        ];
        for json in variants {
            serde_json::from_str::<Transaction>(json).unwrap();
        }
    }
}
