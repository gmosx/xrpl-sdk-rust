use crate::serialize::{Serialize, Serializer};
use crate::{AccountId, Blob, Hash128, Hash256, TransactionCommon, TransactionType, UInt32, UInt8};
use enumflags2::{bitflags, BitFlags};

/// An `AccountSet` transaction <https://xrpl.org/accountset.html>
#[derive(Debug, Clone)]
pub struct AccountSetTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<AccountSetTransactionFlags>,
    pub clear_flag: Option<AccountSetFlag>,
    pub domain: Option<Blob>,
    pub email_hash: Option<Hash128>,
    pub message_key: Option<Blob>,
    pub nf_token_minter: Option<Blob>,
    pub set_flag: Option<AccountSetFlag>,
    pub transfer_rate: Option<UInt32>,
    pub tick_size: Option<UInt8>,
    pub wallet_locator: Option<Hash256>,
    pub wallet_size: Option<UInt32>,
}

impl AccountSetTransaction {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            nf_token_minter: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: None,
            wallet_locator: None,
            wallet_size: None,
        }
    }
}

/// `AccountSet` flags <https://xrpl.org/accountset.html#accountset-flags>
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountSetFlag {
    AccountTxnID = 5,
    AuthorizedNFTokenMinter = 10,
    DefaultRipple = 8,
    DepositAuth = 9,
    DisableMaster = 4,
    DisallowIncomingCheck = 13,
    DisallowIncomingNFTokenOffer = 12,
    DisallowIncomingPayChan = 14,
    DisallowIncomingTrustline = 15,
    DisallowXRP = 3,
    GlobalFreeze = 7,
    NoFreeze = 6,
    RequireAuth = 2,
    RequireDest = 1,
}

/// `AccountSet` flags <https://xrpl.org/accountset.html#accountset-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountSetTransactionFlags {
    FullyCanonicalSig = 0x80000000,
    RequireDestTag = 0x00010000,
    OptionalDestTag = 0x00020000,
    RequireAuth = 0x00040000,
    OptionalAuth = 0x00080000,
    DisallowXRP = 0x00100000,
    AllowXRP = 0x00200000,
}

impl Serialize for AccountSetTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::AccountSet as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        if let Some(clear_flag) = self.clear_flag {
            s.serialize_uint32("ClearFlag", clear_flag as UInt32)?;
        }
        if let Some(domain) = self.domain.as_ref() {
            s.serialize_blob("Domain", domain)?;
        }
        if let Some(email_hash) = self.email_hash {
            s.serialize_hash128("EmailHash", email_hash)?;
        }
        if let Some(message_key) = self.message_key.as_ref() {
            s.serialize_blob("MessageKey", message_key)?;
        }
        if let Some(nf_token_minter) = self.nf_token_minter.as_ref() {
            s.serialize_blob("NFTokenMinter", nf_token_minter)?;
        }
        if let Some(set_flag) = self.set_flag {
            s.serialize_uint32("SetFlag", set_flag as UInt32)?;
        }
        if let Some(transfer_rate) = self.transfer_rate {
            s.serialize_uint32("TransferRate", transfer_rate)?;
        }
        if let Some(tick_size) = self.tick_size {
            s.serialize_uint8("TickSize", tick_size)?;
        }
        if let Some(wallet_locator) = self.wallet_locator {
            s.serialize_hash256("WalletLocator", wallet_locator)?;
        }
        if let Some(wallet_size) = self.wallet_size {
            s.serialize_uint32("WalletSize", wallet_size)?;
        }
        Ok(())
    }
}
