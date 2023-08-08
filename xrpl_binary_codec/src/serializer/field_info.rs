use std::collections::HashMap;
use std::sync::OnceLock;
use xrpl_types::serialize::{FieldCode, TypeCode};

#[derive(Debug)]
pub struct FieldInfo {
    pub field_type: TypeCode,
    pub field_code: FieldCode,
}

pub fn field_info(field_name: &str) -> Option<&'static FieldInfo> {
    FIELD_INFO
        .get_or_init(|| create_field_info_map())
        .get(field_name)
}

static FIELD_INFO: OnceLock<HashMap<String, FieldInfo>> = OnceLock::new();

macro_rules! insert_field_info {
    ($map:ident, $field_name:literal, $field_code:literal, $field_type:ident) => {
        if $map
            .insert(
                $field_name.to_string(),
                FieldInfo {
                    field_type: TypeCode::$field_type,
                    field_code: FieldCode($field_code),
                },
            )
            .is_some()
        {
            panic!("Field with name {} inserted twice", $field_name);
        }
    };
}

/// Field info taken from FIELDS in <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json>
fn create_field_info_map() -> HashMap<String, FieldInfo> {
    let mut map = HashMap::new();
    insert_field_info!(map, "CloseResolution", 1, UInt8);
    insert_field_info!(map, "Method", 2, UInt8);
    insert_field_info!(map, "TransactionResult", 3, UInt8);
    insert_field_info!(map, "TickSize", 16, UInt8);
    insert_field_info!(map, "UNLModifyDisabling", 17, UInt8);
    insert_field_info!(map, "HookResult", 18, UInt8);
    insert_field_info!(map, "LedgerEntryType", 1, UInt16);
    insert_field_info!(map, "TransactionType", 2, UInt16);
    insert_field_info!(map, "SignerWeight", 3, UInt16);
    insert_field_info!(map, "TransferFee", 4, UInt16);
    insert_field_info!(map, "Version", 16, UInt16);
    insert_field_info!(map, "HookStateChangeCount", 17, UInt16);
    insert_field_info!(map, "HookEmitCount", 18, UInt16);
    insert_field_info!(map, "HookExecutionIndex", 19, UInt16);
    insert_field_info!(map, "HookApiVersion", 20, UInt16);
    insert_field_info!(map, "NetworkID", 1, UInt32);
    insert_field_info!(map, "Flags", 2, UInt32);
    insert_field_info!(map, "SourceTag", 3, UInt32);
    insert_field_info!(map, "Sequence", 4, UInt32);
    insert_field_info!(map, "PreviousTxnLgrSeq", 5, UInt32);
    insert_field_info!(map, "LedgerSequence", 6, UInt32);
    insert_field_info!(map, "CloseTime", 7, UInt32);
    insert_field_info!(map, "ParentCloseTime", 8, UInt32);
    insert_field_info!(map, "SigningTime", 9, UInt32);
    insert_field_info!(map, "Expiration", 10, UInt32);
    insert_field_info!(map, "TransferRate", 11, UInt32);
    insert_field_info!(map, "WalletSize", 12, UInt32);
    insert_field_info!(map, "OwnerCount", 13, UInt32);
    insert_field_info!(map, "DestinationTag", 14, UInt32);
    insert_field_info!(map, "HighQualityIn", 16, UInt32);
    insert_field_info!(map, "HighQualityOut", 17, UInt32);
    insert_field_info!(map, "LowQualityIn", 18, UInt32);
    insert_field_info!(map, "LowQualityOut", 19, UInt32);
    insert_field_info!(map, "QualityIn", 20, UInt32);
    insert_field_info!(map, "QualityOut", 21, UInt32);
    insert_field_info!(map, "StampEscrow", 22, UInt32);
    insert_field_info!(map, "BondAmount", 23, UInt32);
    insert_field_info!(map, "LoadFee", 24, UInt32);
    insert_field_info!(map, "OfferSequence", 25, UInt32);
    insert_field_info!(map, "FirstLedgerSequence", 26, UInt32);
    insert_field_info!(map, "LastLedgerSequence", 27, UInt32);
    insert_field_info!(map, "TransactionIndex", 28, UInt32);
    insert_field_info!(map, "OperationLimit", 29, UInt32);
    insert_field_info!(map, "ReferenceFeeUnits", 30, UInt32);
    insert_field_info!(map, "ReserveBase", 31, UInt32);
    insert_field_info!(map, "ReserveIncrement", 32, UInt32);
    insert_field_info!(map, "SetFlag", 33, UInt32);
    insert_field_info!(map, "ClearFlag", 34, UInt32);
    insert_field_info!(map, "SignerQuorum", 35, UInt32);
    insert_field_info!(map, "CancelAfter", 36, UInt32);
    insert_field_info!(map, "FinishAfter", 37, UInt32);
    insert_field_info!(map, "SignerListID", 38, UInt32);
    insert_field_info!(map, "SettleDelay", 39, UInt32);
    insert_field_info!(map, "TicketCount", 40, UInt32);
    insert_field_info!(map, "TicketSequence", 41, UInt32);
    insert_field_info!(map, "NFTokenTaxon", 42, UInt32);
    insert_field_info!(map, "MintedNFTokens", 43, UInt32);
    insert_field_info!(map, "BurnedNFTokens", 44, UInt32);
    insert_field_info!(map, "HookStateCount", 45, UInt32);
    insert_field_info!(map, "EmitGeneration", 46, UInt32);
    insert_field_info!(map, "IndexNext", 1, UInt64);
    insert_field_info!(map, "IndexPrevious", 2, UInt64);
    insert_field_info!(map, "BookNode", 3, UInt64);
    insert_field_info!(map, "OwnerNode", 4, UInt64);
    insert_field_info!(map, "BaseFee", 5, UInt64);
    insert_field_info!(map, "ExchangeRate", 6, UInt64);
    insert_field_info!(map, "LowNode", 7, UInt64);
    insert_field_info!(map, "HighNode", 8, UInt64);
    insert_field_info!(map, "DestinationNode", 9, UInt64);
    insert_field_info!(map, "Cookie", 10, UInt64);
    insert_field_info!(map, "ServerVersion", 11, UInt64);
    insert_field_info!(map, "NFTokenOfferNode", 12, UInt64);
    insert_field_info!(map, "EmitBurden", 13, UInt64);
    insert_field_info!(map, "HookOn", 16, UInt64);
    insert_field_info!(map, "HookInstructionCount", 17, UInt64);
    insert_field_info!(map, "HookReturnCode", 18, UInt64);
    insert_field_info!(map, "ReferenceCount", 19, UInt64);
    insert_field_info!(map, "EmailHash", 1, Hash128);
    insert_field_info!(map, "TakerPaysCurrency", 1, Hash160);
    insert_field_info!(map, "TakerPaysIssuer", 2, Hash160);
    insert_field_info!(map, "TakerGetsCurrency", 3, Hash160);
    insert_field_info!(map, "TakerGetsIssuer", 4, Hash160);
    insert_field_info!(map, "LedgerHash", 1, Hash256);
    insert_field_info!(map, "ParentHash", 2, Hash256);
    insert_field_info!(map, "TransactionHash", 3, Hash256);
    insert_field_info!(map, "AccountHash", 4, Hash256);
    insert_field_info!(map, "PreviousTxnID", 5, Hash256);
    insert_field_info!(map, "LedgerIndex", 6, Hash256);
    insert_field_info!(map, "WalletLocator", 7, Hash256);
    insert_field_info!(map, "RootIndex", 8, Hash256);
    insert_field_info!(map, "AccountTxnID", 9, Hash256);
    insert_field_info!(map, "NFTokenID", 10, Hash256);
    insert_field_info!(map, "EmitParentTxnID", 11, Hash256);
    insert_field_info!(map, "EmitNonce", 12, Hash256);
    insert_field_info!(map, "EmitHookHash", 13, Hash256);
    insert_field_info!(map, "BookDirectory", 16, Hash256);
    insert_field_info!(map, "InvoiceID", 17, Hash256);
    insert_field_info!(map, "Nickname", 18, Hash256);
    insert_field_info!(map, "Amendment", 19, Hash256);
    insert_field_info!(map, "Digest", 21, Hash256);
    insert_field_info!(map, "Channel", 22, Hash256);
    insert_field_info!(map, "ConsensusHash", 23, Hash256);
    insert_field_info!(map, "CheckID", 24, Hash256);
    insert_field_info!(map, "ValidatedHash", 25, Hash256);
    insert_field_info!(map, "PreviousPageMin", 26, Hash256);
    insert_field_info!(map, "NextPageMin", 27, Hash256);
    insert_field_info!(map, "NFTokenBuyOffer", 28, Hash256);
    insert_field_info!(map, "NFTokenSellOffer", 29, Hash256);
    insert_field_info!(map, "HookStateKey", 30, Hash256);
    insert_field_info!(map, "HookHash", 31, Hash256);
    insert_field_info!(map, "HookNamespace", 32, Hash256);
    insert_field_info!(map, "HookSetTxnID", 33, Hash256);
    insert_field_info!(map, "Amount", 1, Amount);
    insert_field_info!(map, "Balance", 2, Amount);
    insert_field_info!(map, "LimitAmount", 3, Amount);
    insert_field_info!(map, "TakerPays", 4, Amount);
    insert_field_info!(map, "TakerGets", 5, Amount);
    insert_field_info!(map, "LowLimit", 6, Amount);
    insert_field_info!(map, "HighLimit", 7, Amount);
    insert_field_info!(map, "Fee", 8, Amount);
    insert_field_info!(map, "SendMax", 9, Amount);
    insert_field_info!(map, "DeliverMin", 10, Amount);
    insert_field_info!(map, "MinimumOffer", 16, Amount);
    insert_field_info!(map, "RippleEscrow", 17, Amount);
    insert_field_info!(map, "DeliveredAmount", 18, Amount);
    insert_field_info!(map, "NFTokenBrokerFee", 19, Amount);
    insert_field_info!(map, "PublicKey", 1, Blob);
    insert_field_info!(map, "MessageKey", 2, Blob);
    insert_field_info!(map, "SigningPubKey", 3, Blob);
    insert_field_info!(map, "TxnSignature", 4, Blob);
    insert_field_info!(map, "URI", 5, Blob);
    insert_field_info!(map, "Signature", 6, Blob);
    insert_field_info!(map, "Domain", 7, Blob);
    insert_field_info!(map, "FundCode", 8, Blob);
    insert_field_info!(map, "RemoveCode", 9, Blob);
    insert_field_info!(map, "ExpireCode", 10, Blob);
    insert_field_info!(map, "CreateCode", 11, Blob);
    insert_field_info!(map, "MemoType", 12, Blob);
    insert_field_info!(map, "MemoData", 13, Blob);
    insert_field_info!(map, "MemoFormat", 14, Blob);
    insert_field_info!(map, "Fulfillment", 16, Blob);
    insert_field_info!(map, "Condition", 17, Blob);
    insert_field_info!(map, "MasterSignature", 18, Blob);
    insert_field_info!(map, "UNLModifyValidator", 19, Blob);
    insert_field_info!(map, "ValidatorToDisable", 20, Blob);
    insert_field_info!(map, "ValidatorToReEnable", 21, Blob);
    insert_field_info!(map, "HookStateData", 22, Blob);
    insert_field_info!(map, "HookReturnString", 23, Blob);
    insert_field_info!(map, "HookParameterName", 24, Blob);
    insert_field_info!(map, "HookParameterValue", 25, Blob);
    insert_field_info!(map, "Account", 1, AccountId);
    insert_field_info!(map, "Owner", 2, AccountId);
    insert_field_info!(map, "Destination", 3, AccountId);
    insert_field_info!(map, "Issuer", 4, AccountId);
    insert_field_info!(map, "Authorize", 5, AccountId);
    insert_field_info!(map, "Unauthorize", 6, AccountId);
    insert_field_info!(map, "RegularKey", 8, AccountId);
    insert_field_info!(map, "NFTokenMinter", 9, AccountId);
    insert_field_info!(map, "EmitCallback", 10, AccountId);
    insert_field_info!(map, "HookAccount", 16, AccountId);
    insert_field_info!(map, "TransactionMetaData", 2, Object);
    insert_field_info!(map, "CreatedNode", 3, Object);
    insert_field_info!(map, "DeletedNode", 4, Object);
    insert_field_info!(map, "ModifiedNode", 5, Object);
    insert_field_info!(map, "PreviousFields", 6, Object);
    insert_field_info!(map, "FinalFields", 7, Object);
    insert_field_info!(map, "NewFields", 8, Object);
    insert_field_info!(map, "TemplateEntry", 9, Object);
    insert_field_info!(map, "Memo", 10, Object);
    insert_field_info!(map, "SignerEntry", 11, Object);
    insert_field_info!(map, "NFToken", 12, Object);
    insert_field_info!(map, "EmitDetails", 13, Object);
    insert_field_info!(map, "Hook", 14, Object);
    insert_field_info!(map, "Signer", 16, Object);
    insert_field_info!(map, "Majority", 18, Object);
    insert_field_info!(map, "DisabledValidator", 19, Object);
    insert_field_info!(map, "EmittedTxn", 20, Object);
    insert_field_info!(map, "HookExecution", 21, Object);
    insert_field_info!(map, "HookDefinition", 22, Object);
    insert_field_info!(map, "HookParameter", 23, Object);
    insert_field_info!(map, "HookGrant", 24, Object);
    insert_field_info!(map, "Signers", 3, Array);
    insert_field_info!(map, "SignerEntries", 4, Array);
    insert_field_info!(map, "Template", 5, Array);
    insert_field_info!(map, "Necessary", 6, Array);
    insert_field_info!(map, "Sufficient", 7, Array);
    insert_field_info!(map, "AffectedNodes", 8, Array);
    insert_field_info!(map, "Memos", 9, Array);
    insert_field_info!(map, "NFTokens", 10, Array);
    insert_field_info!(map, "Hooks", 11, Array);
    insert_field_info!(map, "Majorities", 16, Array);
    insert_field_info!(map, "DisabledValidators", 17, Array);
    insert_field_info!(map, "HookExecutions", 18, Array);
    insert_field_info!(map, "HookParameters", 19, Array);
    insert_field_info!(map, "HookGrants", 20, Array);
    map
}
