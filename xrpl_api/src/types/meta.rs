use crate::Amount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AffectedNode {
    CreatedNode {
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "NewFields")]
        new_fields: serde_json::Value,
    },
    ModifiedNode {
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "FinalFields")]
        final_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousFields")]
        previous_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousTxnID")]
        previous_txn_id: Option<String>,
        #[serde(rename = "PreviousTxnLgrSeq")]
        previous_txn_lgr_seq: Option<u32>,
    },
    DeletedNode {
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "FinalFields")]
        final_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousFields")]
        previous_fields: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub affected_nodes: Vec<AffectedNode>,
    pub transaction_index: u32,
    pub transaction_result: TransactionResult,
    #[serde(rename = "delivered_amount")]
    pub delivered_amount: Option<Amount>,
}

/// Transaction result <https://xrpl.org/tec-codes.html>
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum TransactionResult {
    tecAMM_ACCOUNT,
    tecAMM_UNFUNDED,
    tecAMM_BALANCE,
    tecAMM_EMPTY,
    tecAMM_FAILED,
    tecAMM_INVALID_TOKENS,
    tecAMM_NOT_EMPTY,
    tecCANT_ACCEPT_OWN_NFTOKEN_OFFER,
    tecCLAIM,
    tecCRYPTOCONDITION_ERROR,
    tecDIR_FULL,
    tecDUPLICATE,
    tecDST_TAG_NEEDED,
    tecEXPIRED,
    tecFAILED_PROCESSING,
    tecFROZEN,
    tecHAS_OBLIGATIONS,
    tecINSUF_RESERVE_LINE,
    tecINSUF_RESERVE_OFFER,
    tecINSUFF_FEE,
    tecINSUFFICIENT_FUNDS,
    tecINSUFFICIENT_PAYMENT,
    tecINSUFFICIENT_RESERVE,
    tecINTERNAL,
    tecINVARIANT_FAILED,
    tecKILLED,
    tecMAX_SEQUENCE_REACHED,
    tecNEED_MASTER_KEY,
    tecNFTOKEN_BUY_SELL_MISMATCH,
    tecNFTOKEN_OFFER_TYPE_MISMATCH,
    tecNO_ALTERNATIVE_KEY,
    tecNO_AUTH,
    tecNO_DST,
    tecNO_DST_INSUF_XRP,
    tecNO_ENTRY,
    tecNO_ISSUER,
    tecNO_LINE,
    tecNO_LINE_INSUF_RESERVE,
    tecNO_LINE_REDUNDANT,
    tecNO_PERMISSION,
    tecNO_REGULAR_KEY,
    tecNO_SUITABLE_NFTOKEN_PAGE,
    tecNO_TARGET,
    tecOBJECT_NOT_FOUND,
    tecOVERSIZE,
    tecOWNERS,
    tecPATH_DRY,
    tecPATH_PARTIAL,
    tecTOO_SOON,
    tecUNFUNDED,
    tecUNFUNDED_ADD,
    tecUNFUNDED_PAYMENT,
    tecUNFUNDED_OFFER,

    tefALREADY,
    tefBAD_ADD_AUTH,
    tefBAD_AUTH,
    tefBAD_AUTH_MASTER,
    tefBAD_LEDGER,
    tefBAD_QUORUM,
    tefBAD_SIGNATURE,
    tefCREATED,
    tefEXCEPTION,
    tefFAILURE,
    tefINTERNAL,
    tefINVARIANT_FAILED,
    tefMASTER_DISABLED,
    tefMAX_LEDGER,
    tefNFTOKEN_IS_NOT_TRANSFERABLE,
    tefNO_AUTH_REQUIRED,
    tefNO_TICKET,
    tefNOT_MULTI_SIGNING,
    tefPAST_SEQ,
    tefTOO_BIG,
    tefWRONG_PRIOR,

    telBAD_DOMAIN,
    telBAD_PATH_COUNT,
    telBAD_PUBLIC_KEY,
    telCAN_NOT_QUEUE,
    telCAN_NOT_QUEUE_BALANCE,
    telCAN_NOT_QUEUE_BLOCKS,
    telCAN_NOT_QUEUE_BLOCKED,
    telCAN_NOT_QUEUE_FEE,
    telCAN_NOT_QUEUE_FULL,
    telFAILED_PROCESSING,
    telINSUF_FEE_P,
    telLOCAL_ERROR,
    telNETWORK_ID_MAKES_TX_NON_CANONICAL,
    telNO_DST_PARTIAL,
    telREQUIRES_NETWORK_ID,
    telWRONG_NETWORK,

    temBAD_AMM_TOKENS,
    temBAD_AMOUNT,
    temBAD_AUTH_MASTER,
    temBAD_CURRENCY,
    temBAD_EXPIRATION,
    temBAD_FEE,
    temBAD_ISSUER,
    temBAD_LIMIT,
    temBAD_NFTOKEN_TRANSFER_FEE,
    temBAD_OFFER,
    temBAD_PATH,
    temBAD_PATH_LOOP,
    temBAD_SEND_XRP_LIMIT,
    temBAD_SEND_XRP_MAX,
    temBAD_SEND_XRP_NO_DIRECT,
    temBAD_SEND_XRP_PARTIAL,
    temBAD_SEND_XRP_PATHS,
    temBAD_SEQUENCE,
    temBAD_SIGNATURE,
    temBAD_SRC_ACCOUNT,
    temBAD_TRANSFER_RATE,
    temCANNOT_PREAUTH_SELF,
    temDST_IS_SRC,
    temDST_NEEDED,
    temINVALID,
    temINVALID_COUNT,
    temINVALID_FLAG,
    temMALFORMED,
    temREDUNDANT,
    temREDUNDANT_SEND_MAX,
    temRIPPLE_EMPTY,
    temBAD_WEIGHT,
    temBAD_SIGNER,
    temBAD_QUORUM,
    temUNCERTAIN,
    temUNKNOWN,
    temDISABLED,

    terFUNDS_SPENT,
    terINSUF_FEE_B,
    terLAST,
    terNO_ACCOUNT,
    terNO_AMM,
    terNO_AUTH,
    terNO_LINE,
    terNO_RIPPLE,
    terOWNERS,
    terPRE_SEQ,
    terPRE_TICKET,
    terQUEUED,
    terRETRY,
    terSUBMITTED,

    tesSUCCESS,

    #[serde(other)]
    Other,
}

/// Category of transaction result <https://xrpl.org/transaction-results.html>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ResultCategory {
    Tec,
    Tef,
    Tel,
    Tem,
    Ter,
    Tes,
    Other,
}

impl TransactionResult {
    pub fn category(&self) -> ResultCategory {
        use TransactionResult::*;
        match self {
            tecAMM_ACCOUNT
            | tecAMM_UNFUNDED
            | tecAMM_BALANCE
            | tecAMM_EMPTY
            | tecAMM_FAILED
            | tecAMM_INVALID_TOKENS
            | tecAMM_NOT_EMPTY
            | tecCANT_ACCEPT_OWN_NFTOKEN_OFFER
            | tecCLAIM
            | tecCRYPTOCONDITION_ERROR
            | tecDIR_FULL
            | tecDUPLICATE
            | tecDST_TAG_NEEDED
            | tecEXPIRED
            | tecFAILED_PROCESSING
            | tecFROZEN
            | tecHAS_OBLIGATIONS
            | tecINSUF_RESERVE_LINE
            | tecINSUF_RESERVE_OFFER
            | tecINSUFF_FEE
            | tecINSUFFICIENT_FUNDS
            | tecINSUFFICIENT_PAYMENT
            | tecINSUFFICIENT_RESERVE
            | tecINTERNAL
            | tecINVARIANT_FAILED
            | tecKILLED
            | tecMAX_SEQUENCE_REACHED
            | tecNEED_MASTER_KEY
            | tecNFTOKEN_BUY_SELL_MISMATCH
            | tecNFTOKEN_OFFER_TYPE_MISMATCH
            | tecNO_ALTERNATIVE_KEY
            | tecNO_AUTH
            | tecNO_DST
            | tecNO_DST_INSUF_XRP
            | tecNO_ENTRY
            | tecNO_ISSUER
            | tecNO_LINE
            | tecNO_LINE_INSUF_RESERVE
            | tecNO_LINE_REDUNDANT
            | tecNO_PERMISSION
            | tecNO_REGULAR_KEY
            | tecNO_SUITABLE_NFTOKEN_PAGE
            | tecNO_TARGET
            | tecOBJECT_NOT_FOUND
            | tecOVERSIZE
            | tecOWNERS
            | tecPATH_DRY
            | tecPATH_PARTIAL
            | tecTOO_SOON
            | tecUNFUNDED
            | tecUNFUNDED_ADD
            | tecUNFUNDED_PAYMENT
            | tecUNFUNDED_OFFER => ResultCategory::Tec,

            tefALREADY
            | tefBAD_ADD_AUTH
            | tefBAD_AUTH
            | tefBAD_AUTH_MASTER
            | tefBAD_LEDGER
            | tefBAD_QUORUM
            | tefBAD_SIGNATURE
            | tefCREATED
            | tefEXCEPTION
            | tefFAILURE
            | tefINTERNAL
            | tefINVARIANT_FAILED
            | tefMASTER_DISABLED
            | tefMAX_LEDGER
            | tefNFTOKEN_IS_NOT_TRANSFERABLE
            | tefNO_AUTH_REQUIRED
            | tefNO_TICKET
            | tefNOT_MULTI_SIGNING
            | tefPAST_SEQ
            | tefTOO_BIG
            | tefWRONG_PRIOR => ResultCategory::Tef,

            telBAD_DOMAIN
            | telBAD_PATH_COUNT
            | telBAD_PUBLIC_KEY
            | telCAN_NOT_QUEUE
            | telCAN_NOT_QUEUE_BALANCE
            | telCAN_NOT_QUEUE_BLOCKS
            | telCAN_NOT_QUEUE_BLOCKED
            | telCAN_NOT_QUEUE_FEE
            | telCAN_NOT_QUEUE_FULL
            | telFAILED_PROCESSING
            | telINSUF_FEE_P
            | telLOCAL_ERROR
            | telNETWORK_ID_MAKES_TX_NON_CANONICAL
            | telNO_DST_PARTIAL
            | telREQUIRES_NETWORK_ID
            | telWRONG_NETWORK => ResultCategory::Tel,

            temBAD_AMM_TOKENS
            | temBAD_AMOUNT
            | temBAD_AUTH_MASTER
            | temBAD_CURRENCY
            | temBAD_EXPIRATION
            | temBAD_FEE
            | temBAD_ISSUER
            | temBAD_LIMIT
            | temBAD_NFTOKEN_TRANSFER_FEE
            | temBAD_OFFER
            | temBAD_PATH
            | temBAD_PATH_LOOP
            | temBAD_SEND_XRP_LIMIT
            | temBAD_SEND_XRP_MAX
            | temBAD_SEND_XRP_NO_DIRECT
            | temBAD_SEND_XRP_PARTIAL
            | temBAD_SEND_XRP_PATHS
            | temBAD_SEQUENCE
            | temBAD_SIGNATURE
            | temBAD_SRC_ACCOUNT
            | temBAD_TRANSFER_RATE
            | temCANNOT_PREAUTH_SELF
            | temDST_IS_SRC
            | temDST_NEEDED
            | temINVALID
            | temINVALID_COUNT
            | temINVALID_FLAG
            | temMALFORMED
            | temREDUNDANT
            | temREDUNDANT_SEND_MAX
            | temRIPPLE_EMPTY
            | temBAD_WEIGHT
            | temBAD_SIGNER
            | temBAD_QUORUM
            | temUNCERTAIN
            | temUNKNOWN
            | temDISABLED => ResultCategory::Tem,

            terFUNDS_SPENT | terINSUF_FEE_B | terLAST | terNO_ACCOUNT | terNO_AMM | terNO_AUTH
            | terNO_LINE | terNO_RIPPLE | terOWNERS | terPRE_SEQ | terPRE_TICKET | terQUEUED
            | terRETRY | terSUBMITTED => ResultCategory::Ter,

            tesSUCCESS => ResultCategory::Tes,

            Other => ResultCategory::Other,
        }
    }
}
