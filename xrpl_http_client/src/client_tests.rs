#[cfg(test)]
mod tests {
    use crate::client::{Client, NFT_DEVNET_URL};
    use xrpl_api::{
        AccountChannelsRequest, AccountCurrenciesRequest, AccountInfoRequest, AccountLinesRequest,
        AccountNftsRequest, AccountOffersRequest, AccountTxRequest, BookOffersRequest, Currency,
        DepositAuthorizedRequest, FeeRequest, GatewayBalancesRequest, GetOfferObjectRequest,
        LedgerClosedRequest, LedgerCurrentRequest, LedgerDataRequest, LedgerEntryRequest,
        LedgerRequest, ManifestRequest, PingRequest, RandomRequest, ServerInfoRequest,
        ServerStateRequest, TransactionEntryRequest, TxRequest, WithRequestPagination,
    };
    use xrpl_types::{Currency, LedgerIndex};

    #[tokio::test]
    async fn client_can_fetch_account_currencies() {
        let client = Client::default();

        let resp = client
            .call(AccountCurrenciesRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_info() {
        let client = Client::default();

        let req = AccountInfoRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59");
        let resp = client.call(req).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_channels() {
        let client = Client::default();

        let resp = client
            .call(AccountChannelsRequest::new(
                "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_lines() {
        let client = Client::default();

        let resp = client
            .call(AccountLinesRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_offers() {
        let client = Client::default();

        let resp = client
            .call(AccountOffersRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_nfts() {
        let client = Client::builder().base_url(NFT_DEVNET_URL).build();

        let resp = client
            .call(AccountNftsRequest::new(
                "rsuHaTvJh1bDmDoxX9QcKP7HEBSBt4XsHx",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_account_transactions() {
        let client = Client::default();

        let req = AccountTxRequest::new("r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59").limit(5);
        let resp = client.call(req).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_gateway_balances() {
        let client = Client::default();

        let resp = client
            .call(GatewayBalancesRequest::new(
                "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_ledger() {
        let client = Client::default();

        let resp = client.call(LedgerRequest::new()).await;

        dbg!(&resp);

        let resp = client.call(LedgerRequest::new().transactions(true)).await;

        dbg!(&resp);

        let resp = client
            .call(LedgerRequest::new().transactions(true).expanded())
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_info_about_the_last_closed_ledger() {
        let client = Client::default();

        let resp = client.call(LedgerClosedRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_info_about_the_current_ledger() {
        let client = Client::default();

        let resp = client.call(LedgerCurrentRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_ledger_data() {
        let client = Client::default();

        let resp = client
            .call(LedgerDataRequest::with_ledger_hash(
                "842B57C1CC0613299A686D3E9F310EC0422C84D3911E5056389AA7E5808A93C8",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_ledger_entries() {
        let client = Client::default();

        let resp = client
            .call(LedgerEntryRequest::offer(
                "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                359,
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_get_an_offer_object() {
        let client = Client::default();

        let resp = client
            .call(GetOfferObjectRequest::new(
                "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                359,
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_book_offers() {
        let client = Client::default();

        let resp = client
            .call(BookOffersRequest::new(
                Currency::xrp(),
                Currency::issued("USD", "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_check_if_deposit_is_authorized() {
        let client = Client::default();

        let resp = client
            .call(DepositAuthorizedRequest::new(
                "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
                "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
            ))
            .await;

        dbg!(&resp);
    }

    // #[tokio::test]
    // async fn client_can_fetch_nft_buy_offers() {
    //     let client = Client::builder().base_url(NFT_DEVNET_URL).build();

    //     let nft_id = "00090000D0B007439B080E9B05BF62403911301A7B1F0CFAA048C0A200000007";
    //     let resp = client.call(NftBuyOffersRequest::new(nft_id)).await;

    //     let resp = resp.expect("error response");

    //     assert_eq!(resp.nft_id, nft_id);
    // }

    // #[tokio::test]
    // async fn client_can_fetch_nft_sell_offers() {
    //     let client = Client::builder().base_url(NFT_DEVNET_URL).build();

    //     let nft_id = "00090000D0B007439B080E9B05BF62403911301A7B1F0CFAA048C0A200000007";
    //     let resp = client.call(NftSellOffersRequest::new(nft_id)).await;

    //     let resp = resp.expect("error response");

    //     assert_eq!(resp.nft_id, nft_id);
    // }

    // #[tokio::test]
    // async fn client_can_find_a_payment_path() {
    //     let client = Client::default();

    //     let resp = client
    //         .call(RipplePathFindRequest::new(
    //             "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
    //             "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
    //             Amount::issued("0.001", "USD", "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
    //         ))
    //         .await;

    //     let resp = resp;

    //     // #TODO investigate why the server returns notSupported?
    //     assert!(matches!(resp, Err(Error::Api(s)) if s == "notSupported"));
    // }

    #[tokio::test]
    async fn client_can_fetch_transaction_entries() {
        let client = Client::default();

        let resp = client
            .call(
                TransactionEntryRequest::new(
                    "DA86C7F1979A010BB5F54C49116697A44D8088F92C9AA3AAE419136FE8275A10",
                )
                .ledger_index(LedgerIndex::Index(73355924)),
            )
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_transactions() {
        let client = Client::default();

        let tx_hash = "C53ECF838647FA5A4C780377025FEC7999AB4182590510CA461444B207AB74A9";

        let resp = client.call(TxRequest::new(tx_hash).binary(false)).await;

        let resp = resp.expect("error response");

        assert_eq!(resp.tx.common().hash, tx_hash);
        assert_eq!(resp.tx.common().ledger_index, Some(56865245));
    }

    #[tokio::test]
    async fn client_can_fetch_manifests() {
        let client = Client::default();

        let resp = client
            .call(ManifestRequest::new(
                "nHUE7npJuqdYxFL93tGZS7CW9DuWNLAxBVjzc2rEbu65eL4iiA6s",
            ))
            .await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_fees() {
        let client = Client::default();

        let resp = client.call(FeeRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_information_about_the_server() {
        let client = Client::default();

        let resp = client.call(ServerInfoRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_the_server_state() {
        let client = Client::default();

        let resp = client.call(ServerStateRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_ping_the_server() {
        let client = Client::default();

        let resp = client.call(PingRequest::new()).await;

        dbg!(&resp);
    }

    #[tokio::test]
    async fn client_can_fetch_a_random_seed() {
        let client = Client::default();

        let resp = client.call(RandomRequest::new()).await;

        dbg!(&resp);
    }
}
